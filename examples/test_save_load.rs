extern crate partio;

use partio::DataWriter;

fn make_data() -> partio::ParticlesSimple {
    // use builder to create defaults
    let builder: partio::ParticlesSimpleBuilder = partio::ParticlesSimpleBuilder::new();
    let mut foo: partio::ParticlesSimple = builder.finalize();
    // add attributes
    foo.add_attribute("position", partio::ParticleAttributeType::VECTOR, 3);
    foo.add_attribute("life", partio::ParticleAttributeType::FLOAT, 2);
    foo.add_attribute("id", partio::ParticleAttributeType::INT, 1);
    // add some particle data
    for i in 0..5 {
        let index: partio::ParticleIndex = foo.add_particle();
        // position
        let pos_0: f32 = 0.1_f32 * i as f32;
        let pos_1: f32 = 0.1_f32 * (i + 1) as f32;
        let pos_2: f32 = 0.1_f32 * (i + 2) as f32;
        foo.data_write(&pos_0);
        foo.data_write(&pos_1);
        foo.data_write(&pos_2);
        // life
        let life_0: f32 = -1.2_f32 + i as f32;
        let life_1: f32 = 10.0_f32;
        foo.data_write(&life_0);
        foo.data_write(&life_1);
        // id
        let id: u32 = index as u32;
        foo.data_write(&id);
    }
    foo // return
}

fn test_save_load(p: &partio::ParticlesSimple, filename: &str) {
    println!("Testing with file '{}'", filename);
    // TODO: Partio::write(filename,*p);
    p.write(filename);
    // TODO: Partio::ParticlesData* pnew=Partio::read(filename);
}

fn main() {
    println!("test_save_load.rs");
    // TODO: See ~/git/github/partio/src/tests/test.cpp
    let foo: partio::ParticlesSimple = make_data();
    println!("{:?}", foo);
    test_save_load(&foo, "test.bgeo");
}
