extern crate partio;

use partio::DataWriter;

fn make_data() -> partio::ParticlesSimple {
    // use builder to create defaults
    let builder: partio::ParticlesSimpleBuilder = partio::ParticlesSimpleBuilder::new();
    let mut foo: partio::ParticlesSimple = builder.finalize();
    // add attributes
    let position_attr: partio::ParticleAttribute =
        foo.add_attribute("position", partio::ParticleAttributeType::VECTOR, 3);
    let life_attr: partio::ParticleAttribute =
        foo.add_attribute("life", partio::ParticleAttributeType::FLOAT, 2);
    let id_attr: partio::ParticleAttribute =
        foo.add_attribute("id", partio::ParticleAttributeType::INT, 1);
    // add some particle data
    for i in 0..5 {
        let index: partio::ParticleIndex = foo.add_particle();
        // position
        let pos_0: f64 = 0.1_f64 * i as f64;
        let pos_1: f64 = 0.1_f64 * (i + 1) as f64;
        let pos_2: f64 = 0.1_f64 * (i + 2) as f64;
        foo.data_write(&pos_0);
        foo.data_write(&pos_1);
        foo.data_write(&pos_2);
        // life
        let life_0: f64 = -1.2_f64 + i as f64;
        let life_1: f64 = 10.0_f64;
        foo.data_write(&life_0);
        foo.data_write(&life_1);
        // id
        let ref mut id: u64 = index as u64;
        foo.data_write(&id);
    }
    foo // return
}

fn main() {
    println!("test_save_load.rs");
    // TODO: See ~/git/github/partio/src/tests/test.cpp
    let foo: partio::ParticlesSimple = make_data();
    println!("{:?}", foo);
}
