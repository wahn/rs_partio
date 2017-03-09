//! # partio
//!
//! **Rust** implementation of the **C++** library partio (copyright
//! 2010-2011 Disney Enterprises, Inc. All rights reserved).
//!
//! See https://github.com/wdas/partio for the original **source code**
//! repository.
//!
//! This is the initial source code release of partio a tool we used
//! for particle reading/writing.
//!
//! ## Class Model
//!
//! The goal of the library is to abstract the particle interface from
//! the data representation. That is why Partio represents particles
//! using three classes that inherit and provide more functionality
//!
//! * ParticlesInfo - Information about # of particles and attributes
//! * ParticlesData - Read only access to all particle data
//! * ParticlesDataMutable - Read/write access to all particle data
//!
//! ## Using partio
//!
//! ```rust
//! extern crate partio;
//!
//! use partio::DataWriter;
//!
//! fn make_data() -> partio::ParticlesSimple {
//!     // use builder to create defaults
//!     let builder: partio::ParticlesSimpleBuilder = partio::ParticlesSimpleBuilder::new();
//!     let mut foo: partio::ParticlesSimple = builder.finalize();
//!     // add attributes
//!     foo.add_attribute("position", partio::ParticleAttributeType::VECTOR, 3);
//!     foo.add_attribute("life", partio::ParticleAttributeType::FLOAT, 2);
//!     foo.add_attribute("id", partio::ParticleAttributeType::INT, 1);
//!     // add some particle data
//!     for i in 0..5 {
//!         let index: partio::ParticleIndex = foo.add_particle();
//!         // position
//!         let pos_0: f64 = 0.1_f64 * (i + 0) as f64;
//!         let pos_1: f64 = 0.1_f64 * (i + 1) as f64;
//!         let pos_2: f64 = 0.1_f64 * (i + 2) as f64;
//!         foo.data_write(&pos_0);
//!         foo.data_write(&pos_1);
//!         foo.data_write(&pos_2);
//!         // life
//!         let life_0: f64 = -1.2_f64 + i as f64;
//!         let life_1: f64 = 10.0_f64;
//!         foo.data_write(&life_0);
//!         foo.data_write(&life_1);
//!         // id
//!         let ref mut id: u64 = index as u64;
//!         foo.data_write(&id);
//!     }
//!     foo // return
//! }
//!
//! fn test_save_load(p: &partio::ParticlesSimple, filename: &str) {
//!     println!("Testing with file '{}'", filename);
//!     match p.write(filename) {
//!         Ok(_) => println!("{} written", filename),
//!         Err(err) => println!("Error: {:?}", err),
//!     }
//! }
//!
//! fn main() {
//!     let foo: partio::ParticlesSimple = make_data();
//!     println!("{:?}", foo);
//!     test_save_load(&foo, "test.bgeo");
//! }
//! ```

extern crate byteorder;

use byteorder::{BigEndian, WriteBytesExt};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::io::Result;
use std::io::prelude::*;
use std::mem;
use std::path::Path;

pub type ParticleIndex = u64;

// #[derive(Debug)]
// pub struct ParticlesInfo {
// }

// #[derive(Debug)]
// pub struct ParticlesData {
// }

// #[derive(Debug)]
// pub struct ParticlesDataMutable {
// }

#[derive(Debug, Copy, Clone)]
pub enum ParticleAttributeType {
    NONE,
    VECTOR,
    FLOAT,
    INT,
    INDEXEDSTR,
}

#[derive(Debug, Copy, Clone)]
pub struct ParticleAttribute {
    ptype: ParticleAttributeType,
    count: u8, // Number of entries, should be 3 if type is VECTOR
    name: &'static str, // Name of attribute
    attribute_index: usize, // user should not use or change
}

#[derive(Debug)]
pub struct FixedAttribute {
    ptype: ParticleAttributeType,
    count: u8, // Number of entries, should be 3 if type is VECTOR
    name: &'static str, // Name of attribute
}

#[derive(Debug)]
pub struct ParticlesSimple {
    particle_count: u64,
    attribute_data: Vec<u8>,
    attributes: Vec<ParticleAttribute>,
    attribute_strides: Vec<usize>,
    name_to_attribute: HashMap<&'static str, u64>, // std::map<std::string,int>
    fixed_attributes: Vec<FixedAttribute>,
}

pub trait DataWriter<T> {
    fn data_write(&mut self, data: &T);
}

impl ParticlesSimple {
    /// Number of particles in the structure.
    pub fn num_particles(&self) -> u64 {
        self.particle_count
    }
    /// Number of per-particle attributes.
    pub fn num_attributes(&self) -> usize {
        self.attributes.len()
    }
    /// Number of fixed attributes.
    pub fn num_fixed_attributes(&self) -> usize {
        self.fixed_attributes.len()
    }
    /// Adds an attribute to the particle with the provided name, type and count.
    pub fn add_attribute(&mut self,
                         attribute: &'static str,
                         ptype: ParticleAttributeType,
                         count: u8)
                         -> ParticleAttribute {
        let len = self.attributes.len();
        let attr = ParticleAttribute {
            ptype: ptype,
            count: count,
            name: attribute,
            attribute_index: len,
        };
        self.attributes.push(attr);
        let type_size: usize = match ptype {
            ParticleAttributeType::NONE => 0usize,
            ParticleAttributeType::VECTOR => mem::size_of::<f32>(),
            ParticleAttributeType::FLOAT => mem::size_of::<f32>(),
            ParticleAttributeType::INT => mem::size_of::<u32>(),
            ParticleAttributeType::INDEXEDSTR => mem::size_of::<u32>(),
        };
        let stride: usize = type_size * count as usize;
        self.attribute_strides.push(stride);
        // allocate data
        self.attribute_data = Vec::new();
        attr
    }
    /// Adds a new particle and returns it's index.
    pub fn add_particle(&mut self) -> ParticleIndex {
        let index: ParticleIndex = self.particle_count;
        self.particle_count += 1;
        index
    }
    /// Write particle data to external file.
    pub fn write(&self, filename: &str) -> Result<()> {
        // TODO: See ParticleIO.cpp
        // TODO: extensionIgnoringGz
        let path = Path::new(filename);
        let extension = path.extension();
        let extension: &str = extension.unwrap().to_str().unwrap();
        println!("\"{}\" has extension {:?}", filename, extension);
        // TODO: See writeBGEO in BGEO.cpp
        let mut count_bytes: usize = 0_usize;
        let f = File::create(&path).unwrap();
        {
            let mut writer = BufWriter::new(f);
            // magic
            let magic: String = String::from("Bgeo");
            let bytes: Vec<u8> = magic.into_bytes();
            count_bytes += writer.write(&bytes).unwrap();
            // version_char
            let version_char: String = String::from("V");
            let bytes: Vec<u8> = version_char.into_bytes();
            count_bytes += writer.write(&bytes).unwrap();
            // version
            let version: u32 = 5;
            let mut wtr = vec![];
            try!(wtr.write_u32::<BigEndian>(version));
            let bytes: Vec<u8> = wtr;
            count_bytes += writer.write(&bytes).unwrap();
            // n_points
            let n_points: u32 = self.num_particles() as u32;
            let mut wtr = vec![];
            try!(wtr.write_u32::<BigEndian>(n_points));
            let bytes: Vec<u8> = wtr;
            count_bytes += writer.write(&bytes).unwrap();
            // n_prims
            let n_prims: u32 = 0_u32;
            let mut wtr = vec![];
            try!(wtr.write_u32::<BigEndian>(n_prims));
            let bytes: Vec<u8> = wtr;
            count_bytes += writer.write(&bytes).unwrap();
            // n_point_groups
            let n_point_groups: u32 = 0_u32;
            let mut wtr = vec![];
            try!(wtr.write_u32::<BigEndian>(n_point_groups));
            let bytes: Vec<u8> = wtr;
            count_bytes += writer.write(&bytes).unwrap();
            // n_prim_groups
            let n_prim_groups: u32 = 0_u32;
            let mut wtr = vec![];
            try!(wtr.write_u32::<BigEndian>(n_prim_groups));
            let bytes: Vec<u8> = wtr;
            count_bytes += writer.write(&bytes).unwrap();
            // n_point_attrib
            let n_point_attrib: u32 = (self.num_attributes() - 1_usize) as u32;
            let mut wtr = vec![];
            try!(wtr.write_u32::<BigEndian>(n_point_attrib));
            let bytes: Vec<u8> = wtr;
            count_bytes += writer.write(&bytes).unwrap();
            // n_vertex_attrib
            let n_vertex_attrib: u32 = 0_u32;
            let mut wtr = vec![];
            try!(wtr.write_u32::<BigEndian>(n_vertex_attrib));
            let bytes: Vec<u8> = wtr;
            count_bytes += writer.write(&bytes).unwrap();
            // n_prim_attrib
            let n_prim_attrib: u32 = 0_u32;
            let mut wtr = vec![];
            try!(wtr.write_u32::<BigEndian>(n_prim_attrib));
            let bytes: Vec<u8> = wtr;
            count_bytes += writer.write(&bytes).unwrap();
            // n_attrib
            let n_attrib: u32 = self.num_fixed_attributes() as u32;
            let mut wtr = vec![];
            try!(wtr.write_u32::<BigEndian>(n_attrib));
            let bytes: Vec<u8> = wtr;
            count_bytes += writer.write(&bytes).unwrap();
            // write default values for attributes
            let mut found_position: bool = false;
            let mut num_particle_bytes = 0_usize;
            for i in 0..self.num_attributes() {
                num_particle_bytes += self.attribute_strides[i];
                let attr = self.attributes[i];
                if attr.name == "position" {
                    found_position = true;
                } else {
                    // attr.name
                    let bytes: Vec<u8> = String::from(attr.name).into_bytes();
                    count_bytes += writer.write(&bytes).unwrap();
                    // check particle type
                    let houdini_type: u32;
                    match attr.ptype {
                        ParticleAttributeType::INDEXEDSTR => {
                            println!("TODO: attr.ptype == ParticleAttributeType::INDEXEDSTR");
                            // TODO: size
                            // houdini_type
                            houdini_type = 4_u32;
                            let mut wtr = vec![];
                            try!(wtr.write_u32::<BigEndian>(houdini_type));
                            let bytes: Vec<u8> = wtr;
                            count_bytes += writer.write(&bytes).unwrap();
                            // TODO: numIndexes
                        }
                        _ => {
                            // attr.count
                            let asize: u16 = attr.count as u16;
                            let mut wtr = vec![];
                            try!(wtr.write_u16::<BigEndian>(asize));
                            let bytes: Vec<u8> = wtr;
                            count_bytes += writer.write(&bytes).unwrap();
                            // houdini_type
                            houdini_type = match attr.ptype {
                                ParticleAttributeType::FLOAT => 0_u32,
                                ParticleAttributeType::INT => 1_u32,
                                ParticleAttributeType::VECTOR => 5_u32,
                                _ => 0_u32,
                            };
                            let mut wtr = vec![];
                            try!(wtr.write_u32::<BigEndian>(houdini_type));
                            let bytes: Vec<u8> = wtr;
                            count_bytes += writer.write(&bytes).unwrap();
                            // default values
                            let default_value: u32 = 0_u32;
                            for _ in 0..attr.count {
                                let mut wtr = vec![];
                                try!(wtr.write_u32::<BigEndian>(default_value));
                                let bytes: Vec<u8> = wtr;
                                count_bytes += writer.write(&bytes).unwrap();
                            }
                        }
                    }
                }
            }
            if !found_position {
                println!("Partio: didn't find attr 'position' while trying to write BGEO");
                // TODO? return false;
            }
            let mut slice: &[u8] = &self.attribute_data[..];
            let mut particle: &[u8];
            let mut attribute: &[u8];
            let mut data: &[u8];
            for p in 0..self.num_particles() {
                println!("particle #{}", p + 1);
                let tuple = slice.split_at(num_particle_bytes);
                particle = tuple.0;
                slice = tuple.1;
                for a in 0..self.num_attributes() {
                    let particle_attribute = self.attributes[a];
                    let tuple = particle.split_at(particle_attribute.count as usize * 4_usize);
                    attribute = tuple.0;
                    particle = tuple.1;
                    for c in 0..particle_attribute.count {
                        let tuple = attribute.split_at(4);
                        data = tuple.0;
                        attribute = tuple.1;
                        match particle_attribute.ptype {
                            ParticleAttributeType::NONE => {
                                println!("{}[{}] = NONE", particle_attribute.name, c);
                            }
                            ParticleAttributeType::VECTOR => {
                                let buf = [data[0], data[1], data[2], data[3]];
                                let v_value: f32 = unsafe { std::mem::transmute(buf) };
                                println!("{}[{}] = {} {:?}",
                                         particle_attribute.name,
                                         c,
                                         v_value,
                                         data);
                                let mut wtr = vec![];
                                try!(wtr.write_f32::<BigEndian>(v_value));
                                let bytes: Vec<u8> = wtr;
                                count_bytes += writer.write(&bytes).unwrap();
                                if (particle_attribute.name == "position") & (c == 2) {
                                    // set homogeneous coordinate
                                    let v_value: f32 = 1.0_f32;
                                    let mut wtr = vec![];
                                    try!(wtr.write_f32::<BigEndian>(v_value));
                                    let bytes: Vec<u8> = wtr;
                                    count_bytes += writer.write(&bytes).unwrap();
                                }
                            }
                            ParticleAttributeType::FLOAT => {
                                let buf = [data[0], data[1], data[2], data[3]];
                                let f_value: f32 = unsafe { std::mem::transmute(buf) };
                                println!("{}[{}] = {} {:?}",
                                         particle_attribute.name,
                                         c,
                                         f_value,
                                         data);
                                let mut wtr = vec![];
                                try!(wtr.write_f32::<BigEndian>(f_value));
                                let bytes: Vec<u8> = wtr;
                                count_bytes += writer.write(&bytes).unwrap();
                            }
                            ParticleAttributeType::INT => {
                                let buf = [data[0], data[1], data[2], data[3]];
                                let i_value: u32 = unsafe { std::mem::transmute(buf) };
                                println!("{}[{}] = {} {:?}",
                                         particle_attribute.name,
                                         c,
                                         i_value,
                                         data);
                                let mut wtr = vec![];
                                try!(wtr.write_u32::<BigEndian>(i_value));
                                let bytes: Vec<u8> = wtr;
                                count_bytes += writer.write(&bytes).unwrap();
                            }
                            ParticleAttributeType::INDEXEDSTR => {
                                println!("{}[{}] = {:?}", particle_attribute.name, c, data);
                            }
                        }
                    }
                }
            }
            // TODO: fixed attributes
            // write extra bytes
            let bytes: Vec<u8> = vec![0x00, 0xff];
            count_bytes += writer.write(&bytes).unwrap();
        } // the buffer is flushed once writer goes out of scope
        println!("{} bytes written", count_bytes);
        Ok(())
    }
}

impl<T> DataWriter<T> for ParticlesSimple {
    /// Stores particle data of various types in vector of bytes.
    fn data_write(&mut self, data: &T) {
        unsafe {
            self.attribute_data
                .extend_from_slice(std::slice::from_raw_parts(std::mem::transmute(data),
                                                              std::mem::size_of::<T>()));
        }
    }
}

pub struct ParticlesSimpleBuilder {
}

impl ParticlesSimpleBuilder {
    pub fn new() -> ParticlesSimpleBuilder {
        ParticlesSimpleBuilder {}
    }

    pub fn finalize(self) -> ParticlesSimple {
        let attribute_data: Vec<u8> = Vec::new();
        let name_to_attribute: HashMap<&str, u64> = HashMap::new();
        let attributes: Vec<ParticleAttribute> = Vec::new();
        let attribute_strides: Vec<usize> = Vec::new();
        let fixed_attributes: Vec<FixedAttribute> = Vec::new();
        ParticlesSimple {
            particle_count: 0u64,
            attribute_data: attribute_data,
            name_to_attribute: name_to_attribute,
            attributes: attributes,
            attribute_strides: attribute_strides,
            fixed_attributes: fixed_attributes,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
