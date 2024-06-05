#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0, // 黑色
    Blue = 1, // 蓝色
    Green = 2, // 绿色
    Cyan = 3, // 青色
    Red = 4, // 红色
    Magenta = 5, // 洋红色
    Brown = 6, // 棕色
    LightGray = 7, // 浅灰色
    DarkGray = 8, // 深灰色
    LightBlue = 9, // 浅蓝色
    LightGreen = 10, // 浅绿色
    LightCyan = 11, // 浅青色
    LightRed = 12, // 浅红色
    Pink = 13, // 粉色
    Yellow = 14, // 黄色
    White = 15, // 白色
}// 颜色的枚举

// 间隔

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

//间隔

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

//间隔

// 用于向缓冲区写入字符的 Writer 结构
pub struct Writer {
  column_position: usize, // 当前列的位置
  color_code: ColorCode, // 当前使用的颜色代码
  buffer: &'static mut Buffer, // 缓冲区的引用
}

//间隔

impl Writer {
  // 向缓冲区写入字节的方法
  pub fn write_byte(&mut self, byte: u8) {
      match byte {
          b'\n' => self.new_line(), // 如果是换行符，则调用 new_line 方法
          byte => {
              if self.column_position >= BUFFER_WIDTH {
                  self.new_line(); // 如果当前列已经超过了缓冲区的宽度，则调用 new_line 方法
              }

              let row = BUFFER_HEIGHT - 1; // 获取当前行数
              let col = self.column_position; // 获取当前列数

              let color_code = self.color_code; // 获取当前颜色代码
              // 将新字符及其颜色代码写入到缓冲区的当前位置
              self.buffer.chars[row][col] = ScreenChar {
                  ascii_character: byte,
                  color_code,
              };
              self.column_position += 1; // 更新列位置
          }
      }
  }
  // 处理换行符的方法
  pub fn write_string(&mut self, s: &str) {
    for byte in s.bytes() {
        match byte {
            // 可以是能打印的ASCII码字节，也可以是换行符
            0x20..=0x7e | b'\n' => self.write_byte(byte),
            // 不包含在上述范围之内的字节
            _ => self.write_byte(0xfe),
        }

    }
}
  fn new_line(&mut self) {/* TODO */}
}


// in src/vga_buffer.rs

pub fn print_something() {
  let mut writer = Writer {
      column_position: 0,
      color_code: ColorCode::new(Color::Yellow, Color::Black),
      buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
  };

  writer.write_byte(b'H');
  writer.write_string("ello ");
  writer.write_string("Wörld!");
}