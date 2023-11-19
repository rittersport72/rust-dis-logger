use dis_rs::*;

/*
 * Decode DIS message
 */
pub fn decode(bytes: &[u8]) {
    let result = parse(&bytes);

    if result.is_ok() {
        let pdu_vector = result.unwrap();

        for pdu in pdu_vector {
            let _exercise_id = pdu.header.exercise_id;
            let _version = pdu.header.protocol_version;
            let _time_stamp = pdu.header.time_stamp;

            match pdu.header.pdu_type {
                PduType::StartResume => decode_start_resume(&pdu),
                PduType::StopFreeze => decode_stop_freeze(&pdu),
                PduType::Fire => decode_fire(&pdu),
                PduType::Detonation => decode_detonation(&pdu),
                PduType::ElectromagneticEmission => decode_electromagnetic_emission(&pdu),
                _ => decode_other(&pdu),
            }
        }
    } else {
        let error = result.err();
        if error.is_some() {
            let dis_error = error.unwrap();
            println!("DIS error {:?}", dis_error);
        }
    }
}

fn decode_start_resume(pdu: &Pdu) {
    if let PduBody::StartResume(pdu) = &pdu.body {
        let world_time = &pdu.real_world_time;

        println!(
            "StartResume {}:{}",
            world_time.hour, world_time.time_past_hour
        );

        println!("\n");
    }
}

fn decode_stop_freeze(pdu: &Pdu) {
    if let PduBody::StopFreeze(pdu) = &pdu.body {
        let world_time = &pdu.real_world_time;
        let reason = &pdu.reason;

        println!(
            "StopFreeze {}:{} reason {}",
            world_time.hour,
            world_time.time_past_hour,
            reason.to_string()
        );

        println!("\n");
    }
}

fn decode_fire(pdu: &Pdu) {
    if let PduBody::Fire(pdu) = &pdu.body {
        let firing_entity = &pdu.firing_entity_id;
        let target_entity = &pdu.target_entity_id;
        let location = &pdu.location_in_world;

        println!("--- Fire ---");

        println!(
            "    Firing entity {}:{}:{}",
            firing_entity.simulation_address.site_id,
            firing_entity.simulation_address.application_id,
            firing_entity.entity_id
        );

        println!(
            "    Target entity {}:{}:{}",
            target_entity.simulation_address.site_id,
            target_entity.simulation_address.application_id,
            target_entity.entity_id
        );

        println!(
            "    Location {},{},{}",
            location.x_coordinate, location.y_coordinate, location.z_coordinate
        );

        println!("---\n");
    }
}

fn decode_detonation(pdu: &Pdu) {
    if let PduBody::Detonation(pdu) = &pdu.body {
        let target_entity = &pdu.target_entity_id;
        let location = &pdu.location_in_world_coordinates;
        let detonation_result = &pdu.detonation_result;
        let exploding_entity = &pdu.exploding_entity_id;

        println!("### Detonation ###",);

        println!(
            "    Target entity {}:{}:{}",
            target_entity.simulation_address.site_id,
            target_entity.simulation_address.application_id,
            target_entity.entity_id
        );

        println!(
            "    Exploding entity {}:{}:{}",
            exploding_entity.simulation_address.site_id,
            exploding_entity.simulation_address.application_id,
            exploding_entity.entity_id
        );

        println!(
            "    Location {},{},{}",
            location.x_coordinate, location.y_coordinate, location.z_coordinate
        );

        println!("    Detonation result {}", detonation_result);

        println!("###\n");
    }
}

fn decode_electromagnetic_emission(pdu: &Pdu) {
    if let PduBody::ElectromagneticEmission(pdu) = &pdu.body {
        let emitting_entity = &pdu.emitting_entity_id;
        let emitter_systems_len = &pdu.emitter_systems.len();

        println!("))) ElectromagneticEmission )))",);

        println!(
            "    Emitting entity {}:{}:{}",
            emitting_entity.simulation_address.site_id,
            emitting_entity.simulation_address.application_id,
            emitting_entity.entity_id
        );

        println!("    Emitter systems length {}", emitter_systems_len);

        for emitter in &pdu.emitter_systems {
            let emitter_function = &emitter.function;
            let emitter_name = &emitter.name;
            let emitter_number = &emitter.number;

            println!("    + Emitter function {}", emitter_function);
            println!("    + Emitter name {}", emitter_name);
            println!("    + Emitter number {}", emitter_number);

            let beams_length = &emitter.beams.len();
            println!("    + Beams length {}", beams_length);

            for beam in &emitter.beams {
                let track_jam_len = &beam.track_jam_data.len();
                println!("    +++ Track jam length {}", track_jam_len);

                for track_jam in &beam.track_jam_data {
                    let track_jam_entity = &track_jam.entity_id;

                    println!(
                        "    +++ Track jam entity {}:{}:{}",
                        track_jam_entity.simulation_address.site_id,
                        track_jam_entity.simulation_address.application_id,
                        track_jam_entity.entity_id
                    );

                    let track_jam_beam = &track_jam.beam;
                    let track_jam_emitter = &track_jam.emitter;

                    println!("    +++ Track jam beam {}", track_jam_beam);
                    println!("    +++ Track jam emitter {}", track_jam_emitter);
                }
            }
        }

        println!(")))\n");
    }
}

fn decode_other(pdu: &Pdu) {
    let _pdu_type = &pdu.header.pdu_type;
}
