use std::default::Default;
use std::fmt::{Debug, Formatter};

use cgmath::*;

pub trait Component {
    fn start();
    fn update();
    fn render();
    fn shutdown();
}

#[derive(Debug)]
pub struct Transform {
    position: Vector3<f32>,
    rotation: Quaternion<f32>,
    scale: Vector3<f32>,
    local_transform: Matrix4<f32>,
}

impl Default for Transform {
    fn default () -> Self {
        Self {
            position: (0.0, 0.0, 0.0).into(),
            scale: (1.0, 1.0, 1.0).into(),
            rotation: Euler::new(Deg(0.0), Deg(0.0), Deg(0.0)).into(),
            local_transform: Matrix4::identity(),
        }
    }
}

impl Transform {
    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = (x, y, z).into();
        self.update_local_transform();
    }
    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.scale = (x, y, z).into();
        self.update_local_transform();
    }
    pub fn set_rotation(&mut self, x: f32, y: f32, z: f32)  {
        self.rotation = Euler::new(Deg(x), Deg(y), Deg(z)).into();
        self.update_local_transform();
    }

    pub fn get_position(&self) -> Vector3<f32> {
        self.position
    }
    pub fn get_rotation(&self) -> Quaternion<f32> {
        self.rotation
    }
    pub fn get_scale(&self) -> Vector3<f32> {
        self.scale
    }

    pub fn get_local_transform(&self) -> Matrix4<f32>{
        self.local_transform
    }

    pub fn update_local_transform(&mut self) {
        let mut m = Matrix4::identity();
        m = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z) * m;
        m = Matrix4::from(self.rotation) * m;
        m = Matrix4::from_translation(self.position) * m;
        self.local_transform = m;
    }
}

pub struct Node<'node> {
    label: Option<&'static str>,
    parent: Option<&'node Node<'node>>,
    children: Vec<Node<'node>>,
    active: bool,

    transform: Transform,
}

impl<'node> Debug for Node<'node> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let parent = if let Some(p) = self.parent {
            p.label
        } else {
            Some("")
        };

        f.debug_struct("Node")
            .field("label", &self.label)
            .field("active", &self.active)
            .field("transform", &self.transform)
            .field("parent", &parent)
            .field("child_count", &self.children.len())
            //.field("render_component", &self.render_component.is_some())
            //.field("light_component", &self.light_component.is_some())
            .finish()
    }
}

impl<'node> Default for Node<'node> {
    fn default() -> Self {
        Self {
            label: None,
            parent: None,
            children: vec![],
            active: true,
            transform: Transform::default(),
            //render_component: None,
            //light_component: None,
        }
    }
}

impl<'node> Node<'node> {
    pub fn new (label: Option<&'static str>) -> Self {
        Self {
            label,
            ..Default::default()
        }
    }
    pub fn add_child(&mut self, node: Node<'node>) {
       self.children.push(node);
    }

    pub fn get_transform (&mut self) -> &mut Transform {
       &mut self.transform
    }

    pub fn update(&mut self, _dt: f32) {
    }

    pub fn set_label(&mut self, label: &'static str) {
        self.label = Some(label);
    }
}

pub struct Scene<'scene> {
    pub root: Node<'scene>,
}

impl<'scene> Scene<'scene> {
    pub fn new() -> Self {
       Self {
            root: Node::default(),
       }
    }
    pub fn get_root(&mut self) -> &'scene mut Node {
        &mut self.root
    }

    pub fn add_node(&mut self, node: Node<'scene>) {
        self.root.add_child(node);
    }

    pub fn update (&mut self, dt: f32) {
        Scene::recursive_update(&mut self.root, dt);
    }

    pub fn recursive_update(node: &mut Node, dt: f32) {
        node.update(dt);
        for child in node.children.iter_mut() {
            Scene::recursive_update(child, dt);
        }
    }
}
