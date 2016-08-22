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
//! fn main() {
//!     let foo: partio::ParticlesSimple = make_data();
//!     println!("{:?}", foo);
//! }
//! ```

use std::collections::HashMap;
use std::mem;

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
    count: i8, // Number of entries, should be 3 if type is VECTOR
    name: &'static str, // Name of attribute
    attribute_index: usize, // user should not use or change
}

#[derive(Debug)]
pub struct FixedAttribute {
    ptype: ParticleAttributeType,
    count: i8, // Number of entries, should be 3 if type is VECTOR
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
    fn data_write(&mut self,
                  data: &T);
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
                         count: i8)
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
            ParticleAttributeType::VECTOR => mem::size_of::<f64>(),
            ParticleAttributeType::FLOAT => mem::size_of::<f64>(),
            ParticleAttributeType::INT => mem::size_of::<u64>(),
            ParticleAttributeType::INDEXEDSTR => mem::size_of::<u64>(),
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
}

impl<T> DataWriter<T> for ParticlesSimple {
    /// Stores particle data of various types in vector of bytes.
    fn data_write(&mut self,
                  data: &T) {
        unsafe {
            self.attribute_data.extend_from_slice(std::slice::from_raw_parts(std::mem::transmute(data),
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

    pub fn finalize(&self) -> ParticlesSimple {
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
