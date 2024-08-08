use clap::{Arg, ArgMatches, Command};

use crate::conways::point::Point;

pub fn parse_points_arguments(args: String) -> Result<Vec<Point>, String> {
    if args.is_empty() {
        return Err("No arguments provided".into());
    }

    let mut points = Vec::new();
    let args = args.split(":");

    for point in args {
        points.push(Point::try_from(point)?);
    }

    Ok(points)
}

pub fn receive_command_line_arguments() -> Result<ArgMatches, String> {
    let args = Command::new(" Conway's game of life")
        .arg(
            Arg::new("p")
                .short('p')
                .long("points")
                .value_name("Points")
                .help("Specify the points in the format 0,0:1,0:1,1:...."),
        )
        .arg(
            Arg::new("r")
                .short('r')
                .long("Random")
                .value_name("Random")
                .help("Specify the range in the format max_value,min_value,ammout_of_points"),
        )
        .after_help("Don't use -r and -p at the same time")
        .get_matches();

    if args.contains_id("p") && args.contains_id("r") {
        return Err("Don't use -r and -p at the same time".into());
    }

    Ok(args)
}

pub fn parse_random_arguments(args: String) -> Result<Vec<Point>, String> {
    if args.is_empty() {
        return Err("No arguments provided".into());
    }

    let mut args = args.split(",");

    let (Some(max_position), Some(min_position), Some(ammount_of_points), None) =
        (args.next(), args.next(), args.next(), args.next())
    else {
        return Err("Bad arguments format, it should be in the form of max_position,min_position,ammount_of_points".into());
    };

    let Ok(max_position) = max_position.parse::<i32>() else {
        return Err("Max position must be an integer".into());
    };

    let Ok(min_position) = min_position.parse::<i32>() else {
        return Err("Min position must be an integer".into());
    };

    let Ok(ammount_of_points) = ammount_of_points.parse::<u32>() else {
        return Err("Ammout of points must be a positive number".into());
    };

    if min_position > max_position {
        return Err("Max position must be bigger than min position".into());
    }

    let mut random_points = Vec::new();
    let random_range = min_position..max_position;
    for _ in 0..ammount_of_points {
        let random_point = Point::generate_random_point(random_range.clone());
        random_points.push(random_point);
    }

    Ok(random_points)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn can_create_correct_ammount_of_random_points() {
        let random_points = parse_random_arguments("100,0,5".into()).unwrap();

        assert_eq!(random_points.len(), 5);
    }

    #[test]
    fn can_not_create_points_with_missing_arguments() {
        let random_points = parse_random_arguments("100,5".into());

        assert!(random_points.is_err());
    }

    #[test]
    fn can_not_create_points_with_wrong_type_of_arguments() {
        let random_points = parse_random_arguments("hi,100,5".into());

        assert!(random_points.is_err());
    }

    #[test]
    fn can_not_create_points_with_min_value_bigger_than_max_value() {
        let random_points = parse_random_arguments("10,100,5".into());

        assert!(random_points.is_err());
    }

    #[test]
    fn can_not_create_points_with_negative_ammout_of_points() {
        let random_points = parse_random_arguments("10,100,-1".into());

        assert!(random_points.is_err());
    }
}
