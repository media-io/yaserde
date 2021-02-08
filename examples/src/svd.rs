use yaserde_derive::YaSerialize;

#[derive(Default, PartialEq, Debug, YaSerialize)]
struct CpuDef {
  #[yaserde(child)]
  name: String,
  #[yaserde(child)]
  revision: String,
  #[yaserde(child)]
  endian: String, // enum {LE, BE, ME}
  #[yaserde(child)]
  mpupresent: bool,
  #[yaserde(child)]
  fpupresent: bool,
  //#[yaserde(child)]
  //nvicpriobits: enum {8, 16, 32, 64, 128},
  #[yaserde(child)]
  vendorsystickconfig: bool,
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
struct Field {
  name: String,
  #[yaserde(child)]
  description: String,
  #[yaserde(child)]
  bitrange: String,
  #[yaserde(child)]
  access: String,
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
struct Register {
  #[yaserde(child)]
  name: String,
  #[yaserde(child)]
  description: String,
  #[yaserde(child)]
  addressoffset: String,
  #[yaserde(child)]
  size: u8,
  #[yaserde(child)]
  access: String,
  #[yaserde(child)]
  resetvalue: String,
  #[yaserde(child)]
  resetmask: String,
  #[yaserde(child)]
  fields: Vec<Field>,
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
struct Peripheral {
  #[yaserde(child)]
  name: String,
  #[yaserde(child)]
  version: String,
  #[yaserde(child)]
  description: String,
  #[yaserde(child)]
  groupname: String,
  #[yaserde(child)]
  baseaddress: String,
  #[yaserde(child)]
  size: u8,
  #[yaserde(child)]
  access: String,
  #[yaserde(child)]
  registers: Vec<Register>,
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
struct DevAttrs {
  #[yaserde(child)]
  vendor: String,
  #[yaserde(child)]
  vendorid: String,
  #[yaserde(child)]
  name: String,
  #[yaserde(child)]
  series: String,
  #[yaserde(child)]
  version: String,
  #[yaserde(child)]
  description: String,
  #[yaserde(child)]
  licensetext: String,
  #[yaserde(child)]
  cpu: CpuDef,
  #[yaserde(child)]
  addressunitbits: u8,
  #[yaserde(child)]
  width: u8,
  #[yaserde(child)]
  size: u8,
  #[yaserde(child)]
  access: String,
  #[yaserde(child)]
  resetvalue: String,
  #[yaserde(child)]
  resetmask: String,
  #[yaserde(child)]
  peripherals: Vec<Peripheral>,
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
#[yaserde(rename = "device")]
struct Device {
  #[yaserde(attribute)]
  schemaversion: String,
  #[yaserde(attribute)]
  xmlns: String,
  #[yaserde(attribute)]
  xsnonamespaceschemalocation: String,
  #[yaserde(child)]
  devattributes: DevAttrs,
}

#[test]
fn parsing_svd() {
  use std::fs;

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
