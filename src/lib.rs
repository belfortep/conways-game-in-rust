use conways::point::Point;

pub mod conways;

pub fn parse_arguments(args: Vec<String>) -> Result<Vec<Point>, String> {

    if args.is_empty() {
        return Err("No arguments provided".into());
    }
    
    let mut points = Vec::new();
    let args = args[0].split(":");

    for point in args {
        points.push(Point::from(point));
    }

    Ok(points)
}
