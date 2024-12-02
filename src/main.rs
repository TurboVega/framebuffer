// Using small sizes here so we can easily dump the entire buffer.
const SCREEN_WIDTH: usize = 4;
const SCREEN_HEIGHT: usize = 3;
const HORIZONTAL_PAD: usize = 1;
const VERTICAL_PAD: usize = 1;
const HORIZONTAL_EXTRA: usize = HORIZONTAL_PAD * 2;
const VERTICAL_EXTRA: usize = VERTICAL_PAD * 2;
const TOTAL_HEIGHT: usize = SCREEN_HEIGHT + VERTICAL_EXTRA;
const TOTAL_WIDTH: usize = SCREEN_WIDTH + HORIZONTAL_EXTRA;

#[derive(Copy, Clone, Debug)]
struct ScanLine {
    bytes: [u8; TOTAL_WIDTH]
}

impl ScanLine {
    pub fn new() -> Self {
        Self {
            bytes: [0; TOTAL_WIDTH]
        }
    }

    pub fn fill_entire(&mut self, value: u8) {
        self.bytes = [value; TOTAL_WIDTH];
    }

    pub fn set_element(&mut self, x: usize, value: u8) {
        assert!(x <= TOTAL_WIDTH);
        self.bytes[x] = value;
    }

    // Note: This simple copy does not handle clipping!
    pub fn copy_line(&mut self, source: &ScanLine,
                    src_x: usize,
                    dst_x: usize,
                    width: usize) {
        assert!(src_x < TOTAL_WIDTH);
        assert!(dst_x < TOTAL_WIDTH);

        let end_src_x = src_x + width;
        let end_dst_x = dst_x + width;

        assert!(end_src_x <= TOTAL_WIDTH);
        assert!(end_dst_x <= TOTAL_WIDTH); 

        self.bytes[dst_x..end_dst_x].copy_from_slice(&source.bytes[src_x..end_src_x]);
    }
}

#[derive(Copy, Clone, Debug)]
struct FrameBuffer {
    lines: [ScanLine; TOTAL_HEIGHT]
}

impl FrameBuffer {
    pub fn new() -> Self {
        Self {
            lines: [ScanLine::new(); TOTAL_HEIGHT]
        }
    }    

    pub fn fill_entire(&mut self, value: u8) {
        for line in &mut self.lines {
            line.fill_entire(value);
        }
    }

    pub fn set_element(&mut self, x: usize, y: usize, value: u8) {
        assert!(x < TOTAL_WIDTH);
        assert!(y < TOTAL_HEIGHT);
        self.lines[y].set_element(x, value);
    }

    // Note: This simple copy does not handle clipping!
    pub fn copy_block(&mut self, source: &FrameBuffer,
                        src_x: usize, src_y: usize,
                        dst_x: usize, dst_y: usize,
                        width: usize, height: usize) {
        assert!(src_x < TOTAL_WIDTH);
        assert!(src_y < TOTAL_HEIGHT);
        assert!(dst_x < TOTAL_WIDTH);
        assert!(dst_y < TOTAL_HEIGHT);

        let end_src_x = src_x + width;
        let end_src_y = src_y + height;
        let end_dst_x = dst_x + width;
        let end_dst_y = dst_y + height;

        assert!(end_src_x <= TOTAL_WIDTH);
        assert!(end_src_y <= TOTAL_HEIGHT);
        assert!(end_dst_x <= TOTAL_WIDTH);
        assert!(end_dst_y <= TOTAL_HEIGHT);

        let mut src_y = src_y;
        for line in &mut self.lines[dst_y..end_dst_y] {
            line.copy_line(&source.lines[src_y], src_x, dst_x, width);
            src_y += 1;
        }
    }
}

fn main() {
    println!("Simple Frame Buffer Example");
    let mut frame_buffer = FrameBuffer::new();
    println!("\nCreated: {:?}", frame_buffer);
    frame_buffer.fill_entire(0x5E);
    println!("\nFilled: {:?}", frame_buffer);
    frame_buffer.set_element(1, 1, 0x11);
    println!("\nModified: {:?}", frame_buffer);

    let mut other_buffer = FrameBuffer::new();
    other_buffer.fill_entire(0x77);
    
    frame_buffer.copy_block(&other_buffer, 3, 3, 2, 2, 2, 2);
    println!("\nBlock copied: {:?}", frame_buffer);
}
