//! # partio
//!
//! **Rust** implementation of the **C++** library partio (copyright
//! 2010-2011 Disney Enterprises, Inc. All rights reserved). See
//! https://github.com/wdas/partio for the original **source code**
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
//! ParticlesInfo - Information about # of particles and attributes
//! ParticlesData - Read only access to all particle data
//! ParticlesDataMutable - Read/write access to all particle data
//!
//!

use std::collections::HashMap;
use std::mem;

pub type ParticleIndex = u64;

#[derive(Debug)]
pub struct ParticlesInfo {
}

#[derive(Debug)]
pub struct ParticlesData {
}

#[derive(Debug)]
pub struct ParticlesDataMutable {
}

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
    pub attribute_index: usize, // user should not use or change
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
    pub attribute_data: Vec<Box<[u8]>>,
    attributes: Vec<ParticleAttribute>,
    attribute_strides: Vec<usize>,
    name_to_attribute: HashMap<&'static str, u64>, // std::map<std::string,int>
    fixed_attributes: Vec<FixedAttribute>,
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
    /// Adds an attribute to the particle with the provided name, type and count
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
        let data: Vec<u8> = Vec::new();
        let data_pointer = data.into_boxed_slice();
        self.attribute_data.push(data_pointer);
        attr
    }
    /// Adds a new particle and returns it's index
    pub fn add_particle(&mut self) -> ParticleIndex {
        let len = self.attributes.len();
        for i in 0..len {
            // assumes all vectors have the same length
            let stride: usize = self.attribute_strides[i];
            let mut data: Vec<u8> = Vec::new();
            {
                let ref mut data_ref = self.attribute_data[i];
                data.extend_from_slice(data_ref);
                // append stride times zeros
                for _j in 0..stride {
                    data.push(0u8);
                }
            }
            self.attribute_data[i] = data.into_boxed_slice();
        }
        let index: ParticleIndex = self.particle_count;
        self.particle_count += 1;
        index
    }
}

pub struct ParticlesSimpleBuilder {
}

impl ParticlesSimpleBuilder {
    pub fn new() -> ParticlesSimpleBuilder {
        ParticlesSimpleBuilder {}
    }

    pub fn finalize(&self) -> ParticlesSimple {
        let attribute_data: Vec<Box<[u8]>> = Vec::new();
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
