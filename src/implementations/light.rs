use crate::implementations::{
    tuples::*,
    material::*,
    shape::*
};

#[derive(Debug)]
pub struct Light {
    pub intensity: Vec4,
    pub position: Vec4
}

pub fn lighting(material: &Material, shape: &Shape, light: &Light, point: &Vec4, eyev: &Vec4, normalv: &Vec4, intensity: Vec4) -> Vec4 {
    let material_color: Vec4;
    //println!("Material shape: {:?}, Material pattern: {:?}", shape.shape_type, material.pattern);
    if let Some(pattern) = &material.pattern {
        material_color = pattern.color_at(shape, point);
    } else {
        material_color = material.color;
    }
    let effective_color = &material_color * &intensity;
    let lightv = (&light.position - point).normalize();
    let ambient = &effective_color * material.ambient;
    
    let light_dot_normal = lightv.dot(&normalv);

    let diffuse: Vec4;
    let specular: Vec4;
    if light_dot_normal < 0.0 {
        diffuse = color(0.0, 0.0, 0.0);
        specular = color(0.0, 0.0, 0.0);
    } else {
        diffuse = &(&effective_color * material.diffuse) * light_dot_normal;

        let reflectv = (-lightv).reflect(&normalv);
        let reflect_dot_eye = reflectv.dot(&eyev);

        if reflect_dot_eye <= 0.0 {
            specular = color(0.0, 0.0, 0.0);
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = &(&intensity * material.specular) * factor;
        }
    }
    &(&specular + &ambient) + &diffuse
}
