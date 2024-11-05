#[test]
fn parsing_svd() {
  use std::fs;
  use yaserde::YaSerialize;

  #[derive(PartialEq, Debug, YaSerialize)]
  struct CpuDef {
    name: String,
    revision: String,
    endian: String, // enum {LE, BE, ME}
    mpupresent: bool,
    fpupresent: bool,
    //#[yaserde(child)]
    //nvicpriobits: enum {8, 16, 32, 64, 128},
    vendorsystickconfig: bool,
  }

  #[derive(PartialEq, Debug, YaSerialize)]
  struct Field {
    name: String,
    description: String,
    bitrange: String,
    access: String,
  }

  #[derive(PartialEq, Debug, YaSerialize)]
  struct Register {
    name: String,
    description: String,
    addressoffset: String,
    size: u8,
    access: String,
    resetvalue: String,
    resetmask: String,
    fields: Vec<Field>,
  }

  #[derive(PartialEq, Debug, YaSerialize)]
  struct Peripheral {
    name: String,
    version: String,
    description: String,
    groupname: String,
    baseaddress: String,
    size: u8,
    access: String,
    registers: Vec<Register>,
  }

  #[derive(PartialEq, Debug, YaSerialize)]
  struct DevAttrs {
    vendor: String,
    vendorid: String,
    name: String,
    series: String,
    version: String,
    description: String,
    licensetext: String,
    cpu: CpuDef,
    addressunitbits: u8,
    width: u8,
    size: u8,
    access: String,
    resetvalue: String,
    resetmask: String,
    peripherals: Vec<Peripheral>,
  }

  #[derive(PartialEq, Debug, YaSerialize)]
  #[yaserde(rename = "device")]
  struct Device {
    #[yaserde(attribute = true)]
    schemaversion: String,
    #[yaserde(attribute = true)]
    xmlns: String,
    #[yaserde(attribute = true)]
    xsnonamespaceschemalocation: String,
    devattributes: DevAttrs,
  }

  let register = Register {
        name: "PRCMD".to_string(),
        description: "This command register (PRCMD) is to protect the registers that may have a significant influence on the application system (PSC, PSM) from an inadvertent write access, so that the system does not stop in case of a program hang-up.".to_string(),
        addressoffset: "0xFFFFF1FC".to_string(),
        size: 8,
        access: "read-write".to_string(),
        resetvalue: "0x0000".to_string(),
        resetmask: "0xFFFF".to_string(),
        fields: vec![],
    };

  let vec_registers = vec![register];

  let peripheral = Peripheral {
    name: "Specific Registers".to_string(),
    version: "1.0".to_string(),
    description: "Specific Registers".to_string(),
    groupname: "MCU".to_string(),
    baseaddress: "0xFFFFF1FC".to_string(),
    size: 16,
    access: "read-write".to_string(),
    registers: vec_registers,
  };

  let vec_peripherals = vec![peripheral];

  let cpu_def = CpuDef {
    name: "V850".to_string(),
    revision: "r1".to_string(),
    endian: "LE".to_string(), // enum {LE, BE, ME}
    mpupresent: false,
    fpupresent: false,
    //nvicpriobits: enum {8, 16, 32, 64, 128},
    vendorsystickconfig: false,
  };

  let dev_attrs = DevAttrs {
    vendor: "Renesas".to_string(),
    vendorid: "Renesas".to_string(),
    name: "V850".to_string(),
    series: "E1/E2/CA2".to_string(),
    version: "1.2".to_string(),
    description: "NEC/Renesas V850 automotive grade ICs".to_string(),
    licensetext: "GPLv3".to_string(),
    cpu: cpu_def,
    addressunitbits: 8,
    width: 32,
    size: 32,
    access: "read-write".to_string(),
    resetvalue: "0x00000000".to_string(),
    resetmask: "0xFFFFFFFF".to_string(),
    peripherals: vec_peripherals,
  };

  let device = Device {
    schemaversion: "foo".to_string(),
    xmlns: "http://www.w3.org/2001/XMLSchema-instance".to_string(),
    xsnonamespaceschemalocation: "CMSIS-SVD.xsd".to_string(),
    devattributes: dev_attrs,
  };

  // Display pretty printed XML
  let yaserde_cfg = yaserde::ser::Config {
    perform_indent: true,
    ..Default::default()
  };

  let serialized = yaserde::ser::to_string_with_config(&device, &yaserde_cfg).unwrap();

  let reference =
    fs::read_to_string("tests/data/svd.xml").expect("something went wrong reading the file");

  assert_eq!(reference, serialized)
}
