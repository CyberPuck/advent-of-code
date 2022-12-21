mod comm_protocol_decoder {
    use std::fs;

    pub fn get_data_stream_start_index(file_name: String, user_message: bool) -> u32 {
        let data_frame_size = if user_message { 13 } else { 3 };
        let data_string = parse_file(file_name);
        // find data string
        let mut data_frame: Vec<char> = vec![];
        // this is tracking the last element added to the data_frame
        let mut counter = 0;
        for data_packet in data_string.chars() {
            //println!("Counter: {}, Data buffer: {:?}", counter, data_frame);
            if counter > data_frame_size {
                if !vec_has_repeats(&data_frame) {
                    return counter;
                } else {
                    data_frame.remove(0);
                    data_frame.push(data_packet);
                }
            } else {
                data_frame.push(data_packet);
            }
            counter += 1;
        }

        return 0;
    }

    fn parse_file(file_name: String) -> String {
        let file_data = fs::read_to_string(file_name).unwrap();
        let file_lines: Vec<_> = file_data.lines().collect();
        if file_lines.len() > 1 {
            panic!(
                "File simulated data stream should be one line, got {}",
                file_lines.len()
            );
        }
        return file_lines[0].to_string();
    }

    fn vec_has_repeats(data_packets: &Vec<char>) -> bool {
        for index in 0..data_packets.len() {
            let current_packet = data_packets.get(index).unwrap();
            for check_index in 0..data_packets.len() {
                if check_index != index && current_packet == data_packets.get(check_index).unwrap()
                {
                    return true;
                }
            }
        }
        return false;
    }
}

fn main() {
    let start_index =
        comm_protocol_decoder::get_data_stream_start_index("sample1.txt".to_string(), true);
    println!("Data stream start is: {}", start_index);
    let start_index =
        comm_protocol_decoder::get_data_stream_start_index("sample2.txt".to_string(), true);
    println!("Data stream start is: {}", start_index);
    let start_index =
        comm_protocol_decoder::get_data_stream_start_index("sample3.txt".to_string(), true);
    println!("Data stream start is: {}", start_index);
    let start_index =
        comm_protocol_decoder::get_data_stream_start_index("sample4.txt".to_string(), true);
    println!("Data stream start is: {}", start_index);
    let start_index =
        comm_protocol_decoder::get_data_stream_start_index("sample5.txt".to_string(), true);
    println!("Data stream start is: {}", start_index);
    let start_index =
        comm_protocol_decoder::get_data_stream_start_index("input1.txt".to_string(), true);
    println!("Data stream start for input is: {}", start_index);
}
