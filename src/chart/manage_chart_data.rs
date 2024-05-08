use splines::{Interpolation, Key, Spline};

use crate::{RunTimeDate, TrafficChart};

pub fn update_charts_data(runtime_data: &mut RunTimeDate, traffic_chart: &mut TrafficChart){
    #[allow(clippy::cast_precision_loss)]
    let tot_seconds = traffic_chart.ticks as f32;
    traffic_chart.tick += 1;
    
    #[allow(clippy::cast_precision_loss)]
    let out_bytes_entry = -1.0 * (runtime_data.tot_out_bytes - runtime_data.tot_out_bytes_prev) as f32;

    #[allow(clippy::cast_precision_loss)]
    let in_bytes_entry = (runtime_data.tot_in_bytes - runtime_data.tot_in_bytes_prev) as f32;
    #[allow(clippy::cast_precision_loss)]
    let out_packets_entry =
        -1.0 * (runtime_data.tot_out_packets - runtime_data.tot_out_packets_prev) as f32;
    #[allow(clippy::cast_precision_loss)]
    let in_packets_entry = (runtime_data.tot_in_packets - runtime_data.tot_in_packets_prev) as f32;

    let out_bytes_key = Key::new(tot_seconds, out_bytes_entry, Interpolation::Cosine);
    let in_bytes_key = Key::new(tot_seconds, in_bytes_entry, Interpolation::Cosinde);
    let out_packets_key = Key::new(tot_seconds, out_packets_entry, Interpolation::Cosine);
    let in_packets_key = Key::new(tot_seconds, in_packets_entry, Interpolation::Cosine);

    update_spline(&mut traffic_chart.out_bytes, out_bytes_key);
    traffic_chart.min_bytes = get_min(&traffic_chart.out_bytes);
    runtime_data.tot_out_bytes_prev = runtime_data.tot_out_bytes;

    // TODO: Continuoe code line 35
}