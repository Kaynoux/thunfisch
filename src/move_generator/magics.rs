/// Black Magic Bitboards
/// Source: Volker Annus http://www.talkchess.com/forum/viewtopic.php?p=727500&t=64790
/// Explained here https://www.chessprogramming.org/Magic_Bitboards#Black_Magic_Bitboards
/// These are used to index the sliding target lookup table
#[derive(Copy, Clone)]
pub struct Magic {
    pub mask: u64,
    pub factor: u64,
    pub offset: usize,
}

pub const ROOK_MAGICS: [Magic; 64] = [
    Magic {
        mask: 0x0001_0101_0101_017e,
        factor: 0x0028_0077_ffeb_fffe,
        offset: 26304,
    },
    Magic {
        mask: 0x0002_0202_0202_027c,
        factor: 0x2004_0102_0109_7fff,
        offset: 35520,
    },
    Magic {
        mask: 0x0004_0404_0404_047a,
        factor: 0x0010_0200_1005_3fff,
        offset: 38592,
    },
    Magic {
        mask: 0x0008_0808_0808_0876,
        factor: 0x0040_0400_0800_4002,
        offset: 8026,
    },
    Magic {
        mask: 0x0010_1010_1010_106e,
        factor: 0x7fd0_0441_ffff_d003,
        offset: 22196,
    },
    Magic {
        mask: 0x0020_2020_2020_205e,
        factor: 0x4020_0088_87df_fffe,
        offset: 80870,
    },
    Magic {
        mask: 0x0040_4040_4040_403e,
        factor: 0x0040_0088_8847_ffff,
        offset: 76747,
    },
    Magic {
        mask: 0x0080_8080_8080_807e,
        factor: 0x0068_00fb_ff75_fffd,
        offset: 30400,
    },
    Magic {
        mask: 0x0001_0101_0101_7e00,
        factor: 0x0000_2801_0113_ffff,
        offset: 11115,
    },
    Magic {
        mask: 0x0002_0202_0202_7c00,
        factor: 0x0020_0402_01fc_ffff,
        offset: 18205,
    },
    Magic {
        mask: 0x0004_0404_0404_7a00,
        factor: 0x007f_e800_42ff_ffe8,
        offset: 53577,
    },
    Magic {
        mask: 0x0008_0808_0808_7600,
        factor: 0x0000_1800_217f_ffe8,
        offset: 62724,
    },
    Magic {
        mask: 0x0010_1010_1010_6e00,
        factor: 0x0000_1800_073f_ffe8,
        offset: 34282,
    },
    Magic {
        mask: 0x0020_2020_2020_5e00,
        factor: 0x0000_1800_e05f_ffe8,
        offset: 29196,
    },
    Magic {
        mask: 0x0040_4040_4040_3e00,
        factor: 0x0000_1800_602f_ffe8,
        offset: 23806,
    },
    Magic {
        mask: 0x0080_8080_8080_7e00,
        factor: 0x0000_3000_2fff_ffa0,
        offset: 49481,
    },
    Magic {
        mask: 0x0001_0101_017e_0100,
        factor: 0x0030_0018_010b_ffff,
        offset: 2410,
    },
    Magic {
        mask: 0x0002_0202_027c_0200,
        factor: 0x0003_000c_0085_fffb,
        offset: 36498,
    },
    Magic {
        mask: 0x0004_0404_047a_0400,
        factor: 0x0004_0008_0201_0008,
        offset: 24478,
    },
    Magic {
        mask: 0x0008_0808_0876_0800,
        factor: 0x0004_0020_2002_0004,
        offset: 10074,
    },
    Magic {
        mask: 0x0010_1010_106e_1000,
        factor: 0x0001_0020_0200_2001,
        offset: 79315,
    },
    Magic {
        mask: 0x0020_2020_205e_2000,
        factor: 0x0001_0010_0080_1040,
        offset: 51779,
    },
    Magic {
        mask: 0x0040_4040_403e_4000,
        factor: 0x0000_0040_4000_8001,
        offset: 13586,
    },
    Magic {
        mask: 0x0080_8080_807e_8000,
        factor: 0x0000_0068_00cd_fff4,
        offset: 19323,
    },
    Magic {
        mask: 0x0001_0101_7e01_0100,
        factor: 0x0040_2000_1008_0010,
        offset: 70612,
    },
    Magic {
        mask: 0x0002_0202_7c02_0200,
        factor: 0x0000_0800_1004_0010,
        offset: 83652,
    },
    Magic {
        mask: 0x0004_0404_7a04_0400,
        factor: 0x0004_0100_0802_0008,
        offset: 63110,
    },
    Magic {
        mask: 0x0008_0808_7608_0800,
        factor: 0x0000_0400_2020_0200,
        offset: 34496,
    },
    Magic {
        mask: 0x0010_1010_6e10_1000,
        factor: 0x0002_0080_1010_0100,
        offset: 84966,
    },
    Magic {
        mask: 0x0020_2020_5e20_2000,
        factor: 0x0000_0080_2001_0020,
        offset: 54341,
    },
    Magic {
        mask: 0x0040_4040_3e40_4000,
        factor: 0x0000_0080_2020_0040,
        offset: 60421,
    },
    Magic {
        mask: 0x0080_8080_7e80_8000,
        factor: 0x0000_8200_2000_4020,
        offset: 86402,
    },
    Magic {
        mask: 0x0001_017e_0101_0100,
        factor: 0x00ff_fd18_0030_0030,
        offset: 50245,
    },
    Magic {
        mask: 0x0002_027c_0202_0200,
        factor: 0x007f_ff7f_bfd4_0020,
        offset: 76622,
    },
    Magic {
        mask: 0x0004_047a_0404_0400,
        factor: 0x003f_ffbd_0018_0018,
        offset: 84676,
    },
    Magic {
        mask: 0x0008_0876_0808_0800,
        factor: 0x001f_ffde_8018_0018,
        offset: 78757,
    },
    Magic {
        mask: 0x0010_106e_1010_1000,
        factor: 0x000f_ffe0_bfe8_0018,
        offset: 37346,
    },
    Magic {
        mask: 0x0020_205e_2020_2000,
        factor: 0x0001_0000_8020_2001,
        offset: 370,
    },
    Magic {
        mask: 0x0040_403e_4040_4000,
        factor: 0x0003_fffb_ff98_0180,
        offset: 42182,
    },
    Magic {
        mask: 0x0080_807e_8080_8000,
        factor: 0x0001_fffd_ff90_00e0,
        offset: 45385,
    },
    Magic {
        mask: 0x0001_7e01_0101_0100,
        factor: 0x00ff_fefe_ebff_d800,
        offset: 61659,
    },
    Magic {
        mask: 0x0002_7c02_0202_0200,
        factor: 0x007f_fff7_ffc0_1400,
        offset: 12790,
    },
    Magic {
        mask: 0x0004_7a04_0404_0400,
        factor: 0x003f_ffbf_e4ff_e800,
        offset: 16762,
    },
    Magic {
        mask: 0x0008_7608_0808_0800,
        factor: 0x001f_fff0_1fc0_3000,
        offset: 0,
    },
    Magic {
        mask: 0x0010_6e10_1010_1000,
        factor: 0x000f_ffe7_f8bf_e800,
        offset: 38380,
    },
    Magic {
        mask: 0x0020_5e20_2020_2000,
        factor: 0x0007_ffdf_df3f_f808,
        offset: 11098,
    },
    Magic {
        mask: 0x0040_3e40_4040_4000,
        factor: 0x0003_fff8_5fff_a804,
        offset: 21803,
    },
    Magic {
        mask: 0x0080_7e80_8080_8000,
        factor: 0x0001_fffd_75ff_a802,
        offset: 39189,
    },
    Magic {
        mask: 0x007e_0101_0101_0100,
        factor: 0x00ff_ffd7_ffeb_ffd8,
        offset: 58628,
    },
    Magic {
        mask: 0x007c_0202_0202_0200,
        factor: 0x007f_ff75_ff7f_bfd8,
        offset: 44116,
    },
    Magic {
        mask: 0x007a_0404_0404_0400,
        factor: 0x003f_ff86_3fbf_7fd8,
        offset: 78357,
    },
    Magic {
        mask: 0x0076_0808_0808_0800,
        factor: 0x001f_ffbf_dfd7_ffd8,
        offset: 44481,
    },
    Magic {
        mask: 0x006e_1010_1010_1000,
        factor: 0x000f_fff8_1028_0028,
        offset: 64134,
    },
    Magic {
        mask: 0x005e_2020_2020_2000,
        factor: 0x0007_ffd7_f7fe_ffd8,
        offset: 41759,
    },
    Magic {
        mask: 0x003e_4040_4040_4000,
        factor: 0x0003_fffc_0c48_0048,
        offset: 1394,
    },
    Magic {
        mask: 0x007e_8080_8080_8000,
        factor: 0x0001_ffff_afd7_ffd8,
        offset: 40910,
    },
    Magic {
        mask: 0x7e01_0101_0101_0100,
        factor: 0x00ff_ffe4_ffdf_a3ba,
        offset: 66516,
    },
    Magic {
        mask: 0x7c02_0202_0202_0200,
        factor: 0x007f_ffef_7ff3_d3da,
        offset: 3897,
    },
    Magic {
        mask: 0x7a04_0404_0404_0400,
        factor: 0x003f_ffbf_dfef_f7fa,
        offset: 3930,
    },
    Magic {
        mask: 0x7608_0808_0808_0800,
        factor: 0x001f_ffef_f7fb_fc22,
        offset: 72934,
    },
    Magic {
        mask: 0x6e10_1010_1010_1000,
        factor: 0x0000_0204_0800_1001,
        offset: 72662,
    },
    Magic {
        mask: 0x5e20_2020_2020_2000,
        factor: 0x0007_fffe_ffff_77fd,
        offset: 56325,
    },
    Magic {
        mask: 0x3e40_4040_4040_4000,
        factor: 0x0003_ffff_bf7d_feec,
        offset: 66501,
    },
    Magic {
        mask: 0x7e80_8080_8080_8000,
        factor: 0x0001_ffff_9dff_a333,
        offset: 14826,
    },
];

pub const BISHOP_MAGICS: [Magic; 64] = [
    Magic {
        mask: 0x0040_2010_0804_0200,
        factor: 0x007f_bfbf_bfbf_bfff,
        offset: 5378,
    },
    Magic {
        mask: 0x0000_4020_1008_0400,
        factor: 0x0000_a060_4010_07fc,
        offset: 4093,
    },
    Magic {
        mask: 0x0000_0040_2010_0a00,
        factor: 0x0001_0040_0802_0000,
        offset: 4314,
    },
    Magic {
        mask: 0x0000_0000_4022_1400,
        factor: 0x0000_8060_0400_0000,
        offset: 6587,
    },
    Magic {
        mask: 0x0000_0000_0244_2800,
        factor: 0x0000_1004_0000_0000,
        offset: 6491,
    },
    Magic {
        mask: 0x0000_0002_0408_5000,
        factor: 0x0000_21c1_00b2_0000,
        offset: 6330,
    },
    Magic {
        mask: 0x0000_0204_0810_2000,
        factor: 0x0000_0400_4100_8000,
        offset: 5609,
    },
    Magic {
        mask: 0x0002_0408_1020_4000,
        factor: 0x0000_0fb0_203f_ff80,
        offset: 22236,
    },
    Magic {
        mask: 0x0020_1008_0402_0000,
        factor: 0x0000_0401_0040_1004,
        offset: 6106,
    },
    Magic {
        mask: 0x0040_2010_0804_0000,
        factor: 0x0000_0200_8020_0802,
        offset: 5625,
    },
    Magic {
        mask: 0x0000_4020_100a_0000,
        factor: 0x0000_0040_1020_2000,
        offset: 16785,
    },
    Magic {
        mask: 0x0000_0040_2214_0000,
        factor: 0x0000_0080_6004_0000,
        offset: 16817,
    },
    Magic {
        mask: 0x0000_0002_4428_0000,
        factor: 0x0000_0044_0200_0000,
        offset: 6842,
    },
    Magic {
        mask: 0x0000_0204_0850_0000,
        factor: 0x0000_0008_0100_8000,
        offset: 7003,
    },
    Magic {
        mask: 0x0002_0408_1020_0000,
        factor: 0x0000_07ef_e0bf_ff80,
        offset: 4197,
    },
    Magic {
        mask: 0x0004_0810_2040_0000,
        factor: 0x0000_0008_2082_0020,
        offset: 7356,
    },
    Magic {
        mask: 0x0010_0804_0200_0200,
        factor: 0x0000_4000_8080_8080,
        offset: 4602,
    },
    Magic {
        mask: 0x0020_1008_0400_0400,
        factor: 0x0002_1f01_0040_0808,
        offset: 4538,
    },
    Magic {
        mask: 0x0040_2010_0a00_0a00,
        factor: 0x0001_8000_c06f_3fff,
        offset: 29531,
    },
    Magic {
        mask: 0x0000_4022_1400_1400,
        factor: 0x0000_2582_0080_1000,
        offset: 45393,
    },
    Magic {
        mask: 0x0000_0244_2800_2800,
        factor: 0x0000_2400_8084_0000,
        offset: 12420,
    },
    Magic {
        mask: 0x0002_0408_5000_5000,
        factor: 0x0000_1800_0c03_fff8,
        offset: 15763,
    },
    Magic {
        mask: 0x0004_0810_2000_2000,
        factor: 0x0000_0a58_4020_8020,
        offset: 5050,
    },
    Magic {
        mask: 0x0008_1020_4000_4000,
        factor: 0x0000_0200_0820_8020,
        offset: 4346,
    },
    Magic {
        mask: 0x0008_0402_0002_0400,
        factor: 0x0000_8040_0081_0100,
        offset: 6074,
    },
    Magic {
        mask: 0x0010_0804_0004_0800,
        factor: 0x0001_0119_0080_2008,
        offset: 7866,
    },
    Magic {
        mask: 0x0020_100a_000a_1000,
        factor: 0x0000_8040_0081_0100,
        offset: 32139,
    },
    Magic {
        mask: 0x0040_2214_0014_2200,
        factor: 0x0001_0040_3c04_03ff,
        offset: 57673,
    },
    Magic {
        mask: 0x0002_4428_0028_4400,
        factor: 0x0007_8402_a880_2000,
        offset: 55365,
    },
    Magic {
        mask: 0x0004_0850_0050_0800,
        factor: 0x0000_1010_0080_4400,
        offset: 15818,
    },
    Magic {
        mask: 0x0008_1020_0020_1000,
        factor: 0x0000_0808_0010_4100,
        offset: 5562,
    },
    Magic {
        mask: 0x0010_2040_0040_2000,
        factor: 0x0000_4004_c008_2008,
        offset: 6390,
    },
    Magic {
        mask: 0x0004_0200_0204_0800,
        factor: 0x0001_0101_2000_8020,
        offset: 7930,
    },
    Magic {
        mask: 0x0008_0400_0408_1000,
        factor: 0x0000_8080_9a00_4010,
        offset: 13329,
    },
    Magic {
        mask: 0x0010_0a00_0a10_2000,
        factor: 0x0007_fefe_0881_0010,
        offset: 7170,
    },
    Magic {
        mask: 0x0022_1400_1422_4000,
        factor: 0x0003_ff0f_833f_c080,
        offset: 27267,
    },
    Magic {
        mask: 0x0044_2800_2844_0200,
        factor: 0x007f_e080_1900_3042,
        offset: 53787,
    },
    Magic {
        mask: 0x0008_5000_5008_0400,
        factor: 0x003f_ffef_ea00_3000,
        offset: 5097,
    },
    Magic {
        mask: 0x0010_2000_2010_0800,
        factor: 0x0000_1010_1000_2080,
        offset: 6643,
    },
    Magic {
        mask: 0x0020_4000_4020_1000,
        factor: 0x0000_8020_0508_0804,
        offset: 6138,
    },
    Magic {
        mask: 0x0002_0002_0408_1000,
        factor: 0x0000_8080_80a8_0040,
        offset: 7418,
    },
    Magic {
        mask: 0x0004_0004_0810_2000,
        factor: 0x0000_1041_0020_0040,
        offset: 7898,
    },
    Magic {
        mask: 0x000a_000a_1020_4000,
        factor: 0x0003_ffdf_7f83_3fc0,
        offset: 42012,
    },
    Magic {
        mask: 0x0014_0014_2240_0000,
        factor: 0x0000_0088_4045_0020,
        offset: 57350,
    },
    Magic {
        mask: 0x0028_0028_4402_0000,
        factor: 0x0000_7ffc_8018_0030,
        offset: 22813,
    },
    Magic {
        mask: 0x0050_0050_0804_0200,
        factor: 0x007f_ffdd_8014_0028,
        offset: 56693,
    },
    Magic {
        mask: 0x0020_0020_1008_0400,
        factor: 0x0002_0080_200a_0004,
        offset: 5818,
    },
    Magic {
        mask: 0x0040_0040_2010_0800,
        factor: 0x0000_1010_1010_0020,
        offset: 7098,
    },
    Magic {
        mask: 0x0000_0204_0810_2000,
        factor: 0x0007_ffdf_c180_5000,
        offset: 4451,
    },
    Magic {
        mask: 0x0000_0408_1020_4000,
        factor: 0x0003_ffef_e0c0_2200,
        offset: 4709,
    },
    Magic {
        mask: 0x0000_0a10_2040_0000,
        factor: 0x0000_0008_2080_6000,
        offset: 4794,
    },
    Magic {
        mask: 0x0000_1422_4000_0000,
        factor: 0x0000_0000_0840_3000,
        offset: 13364,
    },
    Magic {
        mask: 0x0000_2844_0200_0000,
        factor: 0x0000_0001_0020_2000,
        offset: 4570,
    },
    Magic {
        mask: 0x0000_5008_0402_0000,
        factor: 0x0000_0040_4080_2000,
        offset: 4282,
    },
    Magic {
        mask: 0x0000_2010_0804_0200,
        factor: 0x0004_0100_4010_0400,
        offset: 14964,
    },
    Magic {
        mask: 0x0000_4020_1008_0400,
        factor: 0x0000_6020_6018_03f4,
        offset: 4026,
    },
    Magic {
        mask: 0x0002_0408_1020_4000,
        factor: 0x0003_ffdf_dfc2_8048,
        offset: 4826,
    },
    Magic {
        mask: 0x0004_0810_2040_0000,
        factor: 0x0000_0008_2082_0020,
        offset: 7354,
    },
    Magic {
        mask: 0x000a_1020_4000_0000,
        factor: 0x0000_0000_0820_8060,
        offset: 4848,
    },
    Magic {
        mask: 0x0014_2240_0000_0000,
        factor: 0x0000_0000_0080_8020,
        offset: 15946,
    },
    Magic {
        mask: 0x0028_4402_0000_0000,
        factor: 0x0000_0000_0100_2020,
        offset: 14932,
    },
    Magic {
        mask: 0x0050_0804_0200_0000,
        factor: 0x0000_0004_0100_2008,
        offset: 16588,
    },
    Magic {
        mask: 0x0020_1008_0402_0000,
        factor: 0x0000_0040_4040_4040,
        offset: 6905,
    },
    Magic {
        mask: 0x0040_2010_0804_0200,
        factor: 0x007f_ff9f_df7f_f813,
        offset: 16076,
    },
];
