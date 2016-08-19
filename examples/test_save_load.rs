extern crate partio;

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
        // TODO: dataWrite<...>(attr, index)
        let ref mut data_ref = foo.attribute_data;
        println!("{:p}", data_ref as *const _);
        {
            // float* pos=foo.dataWrite<float>(positionAttr,index);
            let ref mut pos = data_ref[position_attr.attribute_index];
            // pos[0]=.1*i;
            let pos_0: f64 = 0.1_f64 * i as f64;
            let raw_bytes: [u8; 8] = unsafe { std::mem::transmute(pos_0) };
            for bi in 0..8 {
                pos[bi + 0 * 8 + 3 * 8 * i] = raw_bytes[bi];
            }
            // pos[1]=.1*(i+1);
            let pos_0: f64 = 0.1_f64 * (i + 1) as f64;
            let raw_bytes: [u8; 8] = unsafe { std::mem::transmute(pos_0) };
            for bi in 0..8 {
                pos[bi + 1 * 8 + 3 * 8 * i] = raw_bytes[bi];
            }
            // pos[2]=.1*(i+2);
            let pos_0: f64 = 0.1_f64 * (i + 2) as f64;
            let raw_bytes: [u8; 8] = unsafe { std::mem::transmute(pos_0) };
            for bi in 0..8 {
                pos[bi + 2 * 8 + 3 * 8 * i] = raw_bytes[bi];
            }
            println!("position data{:?}", pos);
        }
        {
            // float* life=foo.dataWrite<float>(lifeAttr,index);
            let ref mut life = data_ref[life_attr.attribute_index];
            // life[0]=-1.2+i;
            let life_0: f64 = -0.2_f64 + i as f64;
            let raw_bytes: [u8; 8] = unsafe { std::mem::transmute(life_0) };
            for bi in 0..8 {
                life[bi + 0 * 8 + 2 * 8 * i] = raw_bytes[bi];
            }
            // life[1]=10.;
            let life_1: f64 = 10.0_f64;
            let raw_bytes: [u8; 8] = unsafe { std::mem::transmute(life_1) };
            for bi in 0..8 {
                life[bi + 1 * 8 + 2 * 8 * i] = raw_bytes[bi];
            }
            println!("life data{:?}", life);
        }
        {
            // int* id=foo.dataWrite<int>(idAttr,index);
            let ref mut id = data_ref[id_attr.attribute_index];
            // id[0]=index;
            let raw_bytes: [u8; 8] = unsafe { std::mem::transmute(index) };
            for bi in 0..8 {
                id[bi + 8 * i] = raw_bytes[bi];
            }
            println!("id data{:?}", id);
        }
    }
    foo // return
}

fn main() {
    println!("test_save_load.rs");
    // TODO: See ~/git/github/partio/src/tests/test.cpp
    let foo: partio::ParticlesSimple = make_data();
    println!("{:?}", foo);
    let ref data_ref = foo.attribute_data;
    println!("{:p}", data_ref as *const _);
}
