use ray_tracer::implementations::{matrices::*, pattern::*, patterns::checker_pattern::CheckerPattern, patterns::gradient_pattern::GradientPattern, patterns::{ring_pattern::RingPattern, striped_pattern::StripedPattern}, shape::ShapeType, shape::*, tuples::*};

static WHITE: Vec4 = Vec4(1.0, 1.0, 1.0, 0.0);
static BLACK: Vec4 = Vec4(0.0, 0.0, 0.0, 0.0);
#[test]
fn test() {
    let striped_pattern = StripedPattern {
        a: WHITE, 
        b: BLACK, 
        transform: Matrix4x4::ident()
    };

    let shape = Shape::new(ShapeType::Sphere);

    assert_eq!(striped_pattern.color_at(&shape, &point(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(striped_pattern.color_at(&shape, &point(0.0, 1.0, 0.0)), WHITE);
    assert_eq!(striped_pattern.color_at(&shape, &point(0.0, 2.0, 0.0)), WHITE);

    assert_eq!(striped_pattern.color_at(&shape, &point(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(striped_pattern.color_at(&shape, &point(0.0, 0.0, 1.0)), WHITE);
    assert_eq!(striped_pattern.color_at(&shape, &point(0.0, 0.0, 2.0)), WHITE);

    assert_eq!(striped_pattern.color_at(&shape, &point(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(striped_pattern.color_at(&shape, &point(0.9, 0.0, 0.0)), WHITE);
    assert_eq!(striped_pattern.color_at(&shape, &point(1.0, 0.0, 2.0)), BLACK);
    assert_eq!(striped_pattern.color_at(&shape, &point(-0.1, 0.0, 2.0)), BLACK);
    assert_eq!(striped_pattern.color_at(&shape, &point(-1.0, 0.0, 2.0)), BLACK);
    assert_eq!(striped_pattern.color_at(&shape, &point(-1.1, 0.0, 2.0)), WHITE);
}

#[test]
fn stripes_with_object_transformation() {
    let mut shape = Shape::new(ShapeType::Sphere);
    shape.transform = scaling(2.0, 2.0, 2.0);
    shape.material.pattern = Some(Box::new(StripedPattern {
        a: WHITE, 
        b: BLACK, 
        transform: Matrix4x4::ident()
    }));
    if let Some(pattern) = &shape.material.pattern {
        let material_color = pattern.color_at(&shape, &point(1.5, 0.0, 0.0));
        assert_eq!(material_color, WHITE);
    } else {
        panic!("Pattern was null when matching shape material");
    }

    let mut shape = Shape::new(ShapeType::Sphere);
    shape.material.pattern = Some(Box::new(StripedPattern {
        a: WHITE, 
        b: BLACK, 
        transform: scaling(2.0, 2.0, 2.0)
    }));
    if let Some(pattern) = &shape.material.pattern {
        let material_color = pattern.color_at(&shape, &point(1.5, 0.0, 0.0));
        assert_eq!(material_color, WHITE);
    } else {
        panic!("Pattern was null when matching shape material");
    }

    let mut shape = Shape::new(ShapeType::Sphere);
    shape.material.pattern = Some(Box::new(StripedPattern {
        a: WHITE, 
        b: BLACK, 
        transform: translation(0.5, 0.0, 0.0)
    }));
    shape.transform = scaling(2.0, 2.0, 2.0);
    if let Some(pattern) = &shape.material.pattern {
        let material_color = pattern.color_at(&shape, &point(2.5, 0.0, 0.0));
        assert_eq!(material_color, WHITE);
    } else {
        panic!("Pattern was null when matching shape material");
    }
}

#[test]
fn gradient_pattern() {
    let pattern = GradientPattern {
        a: WHITE,
        b: BLACK,
        transform: Matrix4x4::ident()
    };
    let shape = Shape::new(ShapeType::Sphere);
    assert_eq!(pattern.color_at(&shape, &point(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.color_at(&shape, &point(0.25, 0.0, 0.0)), color(0.75, 0.75, 0.75));
    assert_eq!(pattern.color_at(&shape, &point(0.5, 0.0, 0.0)), color(0.5, 0.5, 0.5));
    assert_eq!(pattern.color_at(&shape, &point(0.75, 0.0, 0.0)), color(0.25, 0.25, 0.25));
}

#[test]
fn ring_patterns() {
    let pattern = RingPattern {
        a: WHITE,
        b: BLACK,
        transform: Matrix4x4::ident()
    };
    let shape = Shape::new(ShapeType::Sphere);
    assert_eq!(pattern.color_at(&shape, &point(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.color_at(&shape, &point(1.0, 0.0, 0.0)), BLACK);
    assert_eq!(pattern.color_at(&shape, &point(0.0, 0.0, 1.0)), BLACK);

    assert_eq!(pattern.color_at(&shape, &point(0.708, 0.0, 0.708)), BLACK);
}

#[test]
fn checker_pattern() {
    let pattern = CheckerPattern {
        a: WHITE,
        b: BLACK,
        transform: Matrix4x4::ident()
    };
    let shape = Shape::new(ShapeType::Sphere);
    assert_eq!(pattern.color_at(&shape, &point(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.color_at(&shape, &point(0.99, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.color_at(&shape, &point(1.01, 0.0, 0.0)), BLACK);

    assert_eq!(pattern.color_at(&shape, &point(0.0, 0.99, 0.0)), WHITE);
    assert_eq!(pattern.color_at(&shape, &point(0.0, 1.01, 0.0)), BLACK);
    
    assert_eq!(pattern.color_at(&shape, &point(0.0, 0.0, 0.99)), WHITE);
    assert_eq!(pattern.color_at(&shape, &point(0.0, 0.0, 1.01)), BLACK);
}