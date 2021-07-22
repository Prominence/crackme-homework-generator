use rand::Rng;

const CHARSET: &[u8] = b"0123456789";

trait Generator {
    fn generate() -> String;
}

struct DumpGenerator;

impl Generator for DumpGenerator {
    fn generate() -> String {
        let mut rng = rand::thread_rng();
        loop {
            let mut key: String = (0..16).map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            }).collect();
            key.insert(4, '-');
            key.insert(9, '-');
            key.insert(14, '-');

            if is_valid_wrapper(&key) {
                return key;
            }
        }
    }
}

pub fn generate_number() -> String {
    DumpGenerator::generate()
}

pub fn is_valid_wrapper(string: &str) -> bool {
    return fun_140001000(string, string.len() as u8) != 0
}

// param_1 - string to check
// param_2 - number of chars
fn fun_140001000(param_1: &str, param_2: u8) -> u8 {
    // if length is not 19
    if param_2 != 0x13 {
        return 0;
    }

    // check for hyphens every 5 symbols
    for i in (4..param_2).step_by(5) {
        if param_1.chars().nth(i as usize).unwrap() != '-' {
            return 0;
        }
    }

    let mut sections = [['0'; 4]; 4];
    let mut section_number = 0;
    let mut section_position = 0;
    // converting input string into vector of sections
    for symbol in param_1.chars() {
        if symbol == '-' {
            continue;
        }
        sections[section_number][section_position] = symbol;
        if section_position == 3 {
            section_number += 1;
            section_position = 0;
        } else {
            section_position += 1;
        }
    }

    let mut sections_sum = [0; 4];
    let mut total_sum = 0;

    // check that every symbol in sections is a digit
    for (index, section) in sections.iter().enumerate() {
        let mut current_section_sum = 0;
        for section_symbol in section {
            if !section_symbol.is_numeric() {
                return 0;
            }
            current_section_sum += *section_symbol as i32 - 0x30;
        }

        // also calculating sums per section and total sum
        total_sum += current_section_sum;
        sections_sum[index] = current_section_sum;
    }

    // check that sum for every section equals to "total sum >> 2"
    for section_sum in sections_sum {
        if section_sum != total_sum >> 2 {
            return 0;
        }
    }

    // check that there is no the same symbol in neighbor sections
    for i in 0..4 {
        if (sections[0][i] != sections[1][i] && (sections[1][i] != sections[2][i])) &&
            (sections[2][i] != sections[3][i]) {
            continue;
        } else {
            return 0;
        }
    }

    // serial number is valid
    return 1;
}

/*

undefined8 FUN_140001000(char *param_1,int param_2)

{
  int iVar1;
  char *pcVar2;
  uint *puVar3;
  ulonglong uVar4;
  uint uVar5;
  ulonglong uVar7;
  uint uVar8;
  uint *puVar9;
  char *pcVar10;
  uint uVar11;
  uint uVar12;
  uint local_18 [4];
  ulonglong uVar6;

  puVar9 = local_18;
  puVar3 = local_18;
  if (param_2 != 0x13) {
    return 0;
  }
  uVar8 = 0;
  uVar4 = 0;
  pcVar2 = param_1 + 4;
  uVar6 = uVar4;
  do {
    if (*pcVar2 != '-') {
      return 0;
    }
    uVar5 = (int)uVar6 + 1;
    uVar6 = (ulonglong)uVar5;
    pcVar2 = pcVar2 + 5;
    uVar7 = uVar4;
    uVar12 = uVar8;
    uVar11 = uVar8;
    pcVar10 = param_1;
  } while (uVar5 < 3);
  do {
    do {
      if (9 < (int)pcVar10[uVar7] - 0x30U) {
        return 0;
      }
      uVar7 = uVar7 + 1;
    } while ((longlong)uVar7 < 4);
    uVar12 = uVar12 + 1;
    iVar1 = (int)pcVar10[3] + (int)pcVar10[2] + (int)pcVar10[1] + -0xc0 + (int)*pcVar10;
    *puVar9 = iVar1;
    uVar11 = uVar11 + iVar1;
    uVar7 = uVar4;
    pcVar10 = pcVar10 + 5;
    puVar9 = (uint *)((int *)puVar9 + 1);
  } while (uVar12 < 4);
  do {
    if (*puVar3 != uVar11 >> 2) {
      return 0;
    }
    uVar8 = uVar8 + 1;
    puVar3 = puVar3 + 1;
    uVar6 = uVar4;
  } while (uVar8 < 4);
  while (((param_1[uVar4] != param_1[uVar4 + 5] && (param_1[uVar4 + 5] != param_1[uVar4 + 10])) &&
         (param_1[uVar4 + 10] != param_1[uVar4 + 0xf]))) {
    uVar8 = (int)uVar6 + 1;
    uVar4 = uVar4 + 1;
    uVar6 = (ulonglong)uVar8;
    if (3 < uVar8) {
      return 1;
    }
  }
  return 0;
}


undefined8 FUN_140001110(HWND param_1,int param_2,short param_3,HINSTANCE param_4)

{
  UINT UVar1;
  undefined8 uVar2;
  HICON lParam;
  CHAR local_58 [80];

  if (param_2 == 0x10) {
    EndDialog(param_1,0);
    return 1;
  }
  if (param_2 != 0x110) {
    if ((param_2 == 0x111) && (param_3 == 0x3e9)) {
      UVar1 = GetDlgItemTextA(param_1,0x3ea,local_58,0x40);
      uVar2 = FUN_140001000(local_58,UVar1);
      if ((int)uVar2 != 0) {
        MessageBoxA(param_1,"Good work, Serial is valid !!!","Well done...",0x40);
        return 1;
      }
      MessageBoxA(param_1,"Fail, Serial is invalid !!!","Try again...",0x10);
      return 1;
    }
    return 0;
  }
  lParam = LoadIconA(param_4,(LPCSTR)0x65);
  SendMessageA(param_1,0x80,1,(LPARAM)lParam);
  SendMessageA(param_1,0x80,0,(LPARAM)lParam);
  SendMessageA(param_1,0xc,0,(LPARAM)"CrackMe #01");
  return 1;
}

 */

