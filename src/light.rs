use crate::tuples::*;
use crate::material::*;

#[derive(Debug)]
pub struct Light {
    pub intensity: Vec4,
    pub position: Vec4
}

pub fn lighting(material: &Material, light: &Light, point: &Vec4, eyev: &Vec4, normalv: &Vec4) -> Vec4 {
    //println!("Material: {:?} Light: {:?} Point: {:?} Eyev: {:?} Normalv: {:?}", material, light, point, eyev, normalv);
    let effective_color = &material.color * &light.intensity;
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
            specular = &(&light.intensity * material.specular) * factor;
        }
    }

    // amb: 0.1, diffuse: 0.9, specular: 0.9
    if specular.2 > 0.1 {
        println!("Specular {:?}", specular);
    }
    //println!("Specular: {:?} Ambient: {:?} Diffuse: {:?}", specular, ambient, diffuse);
    &(&specular + &ambient) + &diffuse
}