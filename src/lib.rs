use std::collections::HashMap;
use std::path::{Path, PathBuf};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use walkdir::WalkDir;
use rayon::prelude::*;
use plotters::prelude::*;
use anyhow::{Result, anyhow};

#[derive(Debug, Clone)]
struct DirNode {
    name: String,
    path: PathBuf,
    size: u64,
    children: Vec<DirNode>,
    is_file: bool,
}

impl DirNode {
    fn new(name: String, path: PathBuf, is_file: bool) -> Self {
        Self {
            name,
            path,
            size: 0,
            children: Vec::new(),
            is_file,
        }
    }

    fn add_child(&mut self, child: DirNode) {
        self.children.push(child);
    }

    fn calculate_total_size(&mut self) -> u64 {
        if self.is_file {
            return self.size;
        }
        
        self.size = self.children.iter_mut()
            .map(|child| child.calculate_total_size())
            .sum();
        
        self.size
    }
}

fn scan_directory(root_path: &Path) -> Result<DirNode> {
    if !root_path.exists() {
        return Err(anyhow!("Path does not exist: {}", root_path.display()));
    }

    let mut root_node = DirNode::new(
        root_path.file_name()
            .unwrap_or_else(|| std::ffi::OsStr::new("root"))
            .to_string_lossy()
            .to_string(),
        root_path.to_path_buf(),
        false,
    );

    let mut dir_map: HashMap<PathBuf, Vec<DirNode>> = HashMap::new();

    // Process entries and build directory structure
    for entry_result in WalkDir::new(root_path) {
        let entry = entry_result.map_err(|e| anyhow!("Error reading entry: {}", e))?;
        let path = entry.path();
        
        if path == root_path {
            continue;
        }

        let parent = path.parent().unwrap_or(root_path);
        let is_file = entry.file_type().is_file();
        let size = if is_file {
            entry.metadata()
                .map_err(|e| anyhow!("Error reading metadata for {}: {}", path.display(), e))?
                .len()
        } else {
            0
        };

        let node = DirNode {
            name: path.file_name()
                .unwrap_or_else(|| std::ffi::OsStr::new("unknown"))
                .to_string_lossy()
                .to_string(),
            path: path.to_path_buf(),
            size,
            children: Vec::new(),
            is_file,
        };

        dir_map.entry(parent.to_path_buf())
            .or_insert_with(Vec::new)
            .push(node);
    }

    // Build tree structure
    fn build_tree(path: &PathBuf, dir_map: &HashMap<PathBuf, Vec<DirNode>>) -> Vec<DirNode> {
        if let Some(children) = dir_map.get(path) {
            children.iter().map(|child| {
                let mut node = child.clone();
                if !node.is_file {
                    node.children = build_tree(&node.path, dir_map);
                }
                node
            }).collect()
        } else {
            Vec::new()
        }
    }

    root_node.children = build_tree(&root_path.to_path_buf(), &dir_map);
    root_node.calculate_total_size();

    Ok(root_node)
}

#[derive(Debug, Clone)]
struct TreemapRect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    size: u64,
    name: String,
    depth: usize,
}

fn create_treemap_layout(node: &DirNode, x: f64, y: f64, width: f64, height: f64, depth: usize) -> Vec<TreemapRect> {
    let mut rects = Vec::new();
    
    if node.children.is_empty() || node.size == 0 {
        rects.push(TreemapRect {
            x, y, width, height,
            size: node.size,
            name: node.name.clone(),
            depth,
        });
        return rects;
    }

    // Sort children by size (descending)
    let mut children: Vec<_> = node.children.iter().collect();
    children.sort_by(|a, b| b.size.cmp(&a.size));

    let total_size = node.size as f64;
    let mut current_x = x;
    let mut current_y = y;
    let mut remaining_width = width;
    let mut remaining_height = height;

    // Simple horizontal layout for treemap
    for (i, child) in children.iter().enumerate() {
        if child.size == 0 {
            continue;
        }

        let size_ratio = child.size as f64 / total_size;
        
        let (rect_width, rect_height) = if i == children.len() - 1 {
            // Last item gets remaining space
            (remaining_width, remaining_height)
        } else {
            if width > height {
                // Horizontal split
                let rect_width = remaining_width * size_ratio;
                (rect_width, height)
            } else {
                // Vertical split
                let rect_height = remaining_height * size_ratio;
                (width, rect_height)
            }
        };

        let child_rects = create_treemap_layout(
            child, 
            current_x, 
            current_y, 
            rect_width, 
            rect_height, 
            depth + 1
        );
        rects.extend(child_rects);

        if width > height {
            current_x += rect_width;
            remaining_width -= rect_width;
        } else {
            current_y += rect_height;
            remaining_height -= rect_height;
        }
    }

    rects
}

fn generate_treemap_image(root_node: &DirNode, output_path: &str) -> Result<()> {
    let width = 1200u32;
    let height = 800u32;
    
    let drawing_area = BitMapBackend::new(output_path, (width, height))
        .into_drawing_area();
    drawing_area.fill(&WHITE)?;

    let rects = create_treemap_layout(root_node, 0.0, 0.0, width as f64, height as f64, 0);

    // Color palette
    let colors = [
        &RED, &BLUE, &GREEN, &MAGENTA, &CYAN, 
        &BLACK, &RGBColor(255, 165, 0), &RGBColor(128, 0, 128),
        &RGBColor(255, 192, 203), &RGBColor(165, 42, 42),
    ];

    for rect in rects {
        if rect.width < 1.0 || rect.height < 1.0 {
            continue;
        }

        let color = colors[rect.depth % colors.len()];
        let alpha = 0.7 - (rect.depth as f64 * 0.1).min(0.5);
        let fill_color = color.mix(alpha);

        // Draw rectangle
        drawing_area.draw(&Rectangle::new([
            (rect.x as i32, rect.y as i32),
            ((rect.x + rect.width) as i32, (rect.y + rect.height) as i32)
        ], fill_color.stroke_width(2)))?;

        // Draw label if rectangle is large enough
        if rect.width > 60.0 && rect.height > 20.0 {
            let text_x = rect.x + 5.0;
            let text_y = rect.y + 15.0;
            
            let size_str = format_size(rect.size);
            let label = if rect.name.len() > 15 {
                format!("{}...\n{}", &rect.name[..12], size_str)
            } else {
                format!("{}\n{}", rect.name, size_str)
            };

            drawing_area.draw(&Text::new(label, (text_x as i32, text_y as i32), ("Arial", 12).into_font().color(&BLACK)))?;
        }
    }

    drawing_area.present()?;
    Ok(())
}

fn format_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

#[pyfunction]
fn treemap(path: &str, out: &str) -> PyResult<()> {
    let root_path = Path::new(path);
    
    let root_node = scan_directory(root_path)
        .map_err(|e| PyValueError::new_err(format!("Error scanning directory: {}", e)))?;

    generate_treemap_image(&root_node, out)
        .map_err(|e| PyValueError::new_err(format!("Error generating treemap: {}", e)))?;

    Ok(())
}

#[pymodule]
fn visfile(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(treemap, m)?)?;
    Ok(())
}