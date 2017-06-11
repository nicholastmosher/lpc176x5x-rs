# ! [ doc = "CAN acceptance filter RAM" ]

use core::ops::Deref;
use cortex_m::peripheral::Peripheral;

# [ doc = "CAN acceptance filter RAM" ]
pub const CANAFRAM: Peripheral<CANAFRAM> = unsafe { Peripheral::new(1073971200) };
use vcell::VolatileCell;
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct RegisterBlock {
    # [ doc = "0x00 - CAN AF ram access register" ]
    pub mask0: MASK,
    # [ doc = "0x04 - CAN AF ram access register" ]
    pub mask1: MASK,
    # [ doc = "0x08 - CAN AF ram access register" ]
    pub mask2: MASK,
    # [ doc = "0x0c - CAN AF ram access register" ]
    pub mask3: MASK,
    # [ doc = "0x10 - CAN AF ram access register" ]
    pub mask4: MASK,
    # [ doc = "0x14 - CAN AF ram access register" ]
    pub mask5: MASK,
    # [ doc = "0x18 - CAN AF ram access register" ]
    pub mask6: MASK,
    # [ doc = "0x1c - CAN AF ram access register" ]
    pub mask7: MASK,
    # [ doc = "0x20 - CAN AF ram access register" ]
    pub mask8: MASK,
    # [ doc = "0x24 - CAN AF ram access register" ]
    pub mask9: MASK,
    # [ doc = "0x28 - CAN AF ram access register" ]
    pub mask10: MASK,
    # [ doc = "0x2c - CAN AF ram access register" ]
    pub mask11: MASK,
    # [ doc = "0x30 - CAN AF ram access register" ]
    pub mask12: MASK,
    # [ doc = "0x34 - CAN AF ram access register" ]
    pub mask13: MASK,
    # [ doc = "0x38 - CAN AF ram access register" ]
    pub mask14: MASK,
    # [ doc = "0x3c - CAN AF ram access register" ]
    pub mask15: MASK,
    # [ doc = "0x40 - CAN AF ram access register" ]
    pub mask16: MASK,
    # [ doc = "0x44 - CAN AF ram access register" ]
    pub mask17: MASK,
    # [ doc = "0x48 - CAN AF ram access register" ]
    pub mask18: MASK,
    # [ doc = "0x4c - CAN AF ram access register" ]
    pub mask19: MASK,
    # [ doc = "0x50 - CAN AF ram access register" ]
    pub mask20: MASK,
    # [ doc = "0x54 - CAN AF ram access register" ]
    pub mask21: MASK,
    # [ doc = "0x58 - CAN AF ram access register" ]
    pub mask22: MASK,
    # [ doc = "0x5c - CAN AF ram access register" ]
    pub mask23: MASK,
    # [ doc = "0x60 - CAN AF ram access register" ]
    pub mask24: MASK,
    # [ doc = "0x64 - CAN AF ram access register" ]
    pub mask25: MASK,
    # [ doc = "0x68 - CAN AF ram access register" ]
    pub mask26: MASK,
    # [ doc = "0x6c - CAN AF ram access register" ]
    pub mask27: MASK,
    # [ doc = "0x70 - CAN AF ram access register" ]
    pub mask28: MASK,
    # [ doc = "0x74 - CAN AF ram access register" ]
    pub mask29: MASK,
    # [ doc = "0x78 - CAN AF ram access register" ]
    pub mask30: MASK,
    # [ doc = "0x7c - CAN AF ram access register" ]
    pub mask31: MASK,
    # [ doc = "0x80 - CAN AF ram access register" ]
    pub mask32: MASK,
    # [ doc = "0x84 - CAN AF ram access register" ]
    pub mask33: MASK,
    # [ doc = "0x88 - CAN AF ram access register" ]
    pub mask34: MASK,
    # [ doc = "0x8c - CAN AF ram access register" ]
    pub mask35: MASK,
    # [ doc = "0x90 - CAN AF ram access register" ]
    pub mask36: MASK,
    # [ doc = "0x94 - CAN AF ram access register" ]
    pub mask37: MASK,
    # [ doc = "0x98 - CAN AF ram access register" ]
    pub mask38: MASK,
    # [ doc = "0x9c - CAN AF ram access register" ]
    pub mask39: MASK,
    # [ doc = "0xa0 - CAN AF ram access register" ]
    pub mask40: MASK,
    # [ doc = "0xa4 - CAN AF ram access register" ]
    pub mask41: MASK,
    # [ doc = "0xa8 - CAN AF ram access register" ]
    pub mask42: MASK,
    # [ doc = "0xac - CAN AF ram access register" ]
    pub mask43: MASK,
    # [ doc = "0xb0 - CAN AF ram access register" ]
    pub mask44: MASK,
    # [ doc = "0xb4 - CAN AF ram access register" ]
    pub mask45: MASK,
    # [ doc = "0xb8 - CAN AF ram access register" ]
    pub mask46: MASK,
    # [ doc = "0xbc - CAN AF ram access register" ]
    pub mask47: MASK,
    # [ doc = "0xc0 - CAN AF ram access register" ]
    pub mask48: MASK,
    # [ doc = "0xc4 - CAN AF ram access register" ]
    pub mask49: MASK,
    # [ doc = "0xc8 - CAN AF ram access register" ]
    pub mask50: MASK,
    # [ doc = "0xcc - CAN AF ram access register" ]
    pub mask51: MASK,
    # [ doc = "0xd0 - CAN AF ram access register" ]
    pub mask52: MASK,
    # [ doc = "0xd4 - CAN AF ram access register" ]
    pub mask53: MASK,
    # [ doc = "0xd8 - CAN AF ram access register" ]
    pub mask54: MASK,
    # [ doc = "0xdc - CAN AF ram access register" ]
    pub mask55: MASK,
    # [ doc = "0xe0 - CAN AF ram access register" ]
    pub mask56: MASK,
    # [ doc = "0xe4 - CAN AF ram access register" ]
    pub mask57: MASK,
    # [ doc = "0xe8 - CAN AF ram access register" ]
    pub mask58: MASK,
    # [ doc = "0xec - CAN AF ram access register" ]
    pub mask59: MASK,
    # [ doc = "0xf0 - CAN AF ram access register" ]
    pub mask60: MASK,
    # [ doc = "0xf4 - CAN AF ram access register" ]
    pub mask61: MASK,
    # [ doc = "0xf8 - CAN AF ram access register" ]
    pub mask62: MASK,
    # [ doc = "0xfc - CAN AF ram access register" ]
    pub mask63: MASK,
    # [ doc = "0x100 - CAN AF ram access register" ]
    pub mask64: MASK,
    # [ doc = "0x104 - CAN AF ram access register" ]
    pub mask65: MASK,
    # [ doc = "0x108 - CAN AF ram access register" ]
    pub mask66: MASK,
    # [ doc = "0x10c - CAN AF ram access register" ]
    pub mask67: MASK,
    # [ doc = "0x110 - CAN AF ram access register" ]
    pub mask68: MASK,
    # [ doc = "0x114 - CAN AF ram access register" ]
    pub mask69: MASK,
    # [ doc = "0x118 - CAN AF ram access register" ]
    pub mask70: MASK,
    # [ doc = "0x11c - CAN AF ram access register" ]
    pub mask71: MASK,
    # [ doc = "0x120 - CAN AF ram access register" ]
    pub mask72: MASK,
    # [ doc = "0x124 - CAN AF ram access register" ]
    pub mask73: MASK,
    # [ doc = "0x128 - CAN AF ram access register" ]
    pub mask74: MASK,
    # [ doc = "0x12c - CAN AF ram access register" ]
    pub mask75: MASK,
    # [ doc = "0x130 - CAN AF ram access register" ]
    pub mask76: MASK,
    # [ doc = "0x134 - CAN AF ram access register" ]
    pub mask77: MASK,
    # [ doc = "0x138 - CAN AF ram access register" ]
    pub mask78: MASK,
    # [ doc = "0x13c - CAN AF ram access register" ]
    pub mask79: MASK,
    # [ doc = "0x140 - CAN AF ram access register" ]
    pub mask80: MASK,
    # [ doc = "0x144 - CAN AF ram access register" ]
    pub mask81: MASK,
    # [ doc = "0x148 - CAN AF ram access register" ]
    pub mask82: MASK,
    # [ doc = "0x14c - CAN AF ram access register" ]
    pub mask83: MASK,
    # [ doc = "0x150 - CAN AF ram access register" ]
    pub mask84: MASK,
    # [ doc = "0x154 - CAN AF ram access register" ]
    pub mask85: MASK,
    # [ doc = "0x158 - CAN AF ram access register" ]
    pub mask86: MASK,
    # [ doc = "0x15c - CAN AF ram access register" ]
    pub mask87: MASK,
    # [ doc = "0x160 - CAN AF ram access register" ]
    pub mask88: MASK,
    # [ doc = "0x164 - CAN AF ram access register" ]
    pub mask89: MASK,
    # [ doc = "0x168 - CAN AF ram access register" ]
    pub mask90: MASK,
    # [ doc = "0x16c - CAN AF ram access register" ]
    pub mask91: MASK,
    # [ doc = "0x170 - CAN AF ram access register" ]
    pub mask92: MASK,
    # [ doc = "0x174 - CAN AF ram access register" ]
    pub mask93: MASK,
    # [ doc = "0x178 - CAN AF ram access register" ]
    pub mask94: MASK,
    # [ doc = "0x17c - CAN AF ram access register" ]
    pub mask95: MASK,
    # [ doc = "0x180 - CAN AF ram access register" ]
    pub mask96: MASK,
    # [ doc = "0x184 - CAN AF ram access register" ]
    pub mask97: MASK,
    # [ doc = "0x188 - CAN AF ram access register" ]
    pub mask98: MASK,
    # [ doc = "0x18c - CAN AF ram access register" ]
    pub mask99: MASK,
    # [ doc = "0x190 - CAN AF ram access register" ]
    pub mask100: MASK,
    # [ doc = "0x194 - CAN AF ram access register" ]
    pub mask101: MASK,
    # [ doc = "0x198 - CAN AF ram access register" ]
    pub mask102: MASK,
    # [ doc = "0x19c - CAN AF ram access register" ]
    pub mask103: MASK,
    # [ doc = "0x1a0 - CAN AF ram access register" ]
    pub mask104: MASK,
    # [ doc = "0x1a4 - CAN AF ram access register" ]
    pub mask105: MASK,
    # [ doc = "0x1a8 - CAN AF ram access register" ]
    pub mask106: MASK,
    # [ doc = "0x1ac - CAN AF ram access register" ]
    pub mask107: MASK,
    # [ doc = "0x1b0 - CAN AF ram access register" ]
    pub mask108: MASK,
    # [ doc = "0x1b4 - CAN AF ram access register" ]
    pub mask109: MASK,
    # [ doc = "0x1b8 - CAN AF ram access register" ]
    pub mask110: MASK,
    # [ doc = "0x1bc - CAN AF ram access register" ]
    pub mask111: MASK,
    # [ doc = "0x1c0 - CAN AF ram access register" ]
    pub mask112: MASK,
    # [ doc = "0x1c4 - CAN AF ram access register" ]
    pub mask113: MASK,
    # [ doc = "0x1c8 - CAN AF ram access register" ]
    pub mask114: MASK,
    # [ doc = "0x1cc - CAN AF ram access register" ]
    pub mask115: MASK,
    # [ doc = "0x1d0 - CAN AF ram access register" ]
    pub mask116: MASK,
    # [ doc = "0x1d4 - CAN AF ram access register" ]
    pub mask117: MASK,
    # [ doc = "0x1d8 - CAN AF ram access register" ]
    pub mask118: MASK,
    # [ doc = "0x1dc - CAN AF ram access register" ]
    pub mask119: MASK,
    # [ doc = "0x1e0 - CAN AF ram access register" ]
    pub mask120: MASK,
    # [ doc = "0x1e4 - CAN AF ram access register" ]
    pub mask121: MASK,
    # [ doc = "0x1e8 - CAN AF ram access register" ]
    pub mask122: MASK,
    # [ doc = "0x1ec - CAN AF ram access register" ]
    pub mask123: MASK,
    # [ doc = "0x1f0 - CAN AF ram access register" ]
    pub mask124: MASK,
    # [ doc = "0x1f4 - CAN AF ram access register" ]
    pub mask125: MASK,
    # [ doc = "0x1f8 - CAN AF ram access register" ]
    pub mask126: MASK,
    # [ doc = "0x1fc - CAN AF ram access register" ]
    pub mask127: MASK,
    # [ doc = "0x200 - CAN AF ram access register" ]
    pub mask128: MASK,
    # [ doc = "0x204 - CAN AF ram access register" ]
    pub mask129: MASK,
    # [ doc = "0x208 - CAN AF ram access register" ]
    pub mask130: MASK,
    # [ doc = "0x20c - CAN AF ram access register" ]
    pub mask131: MASK,
    # [ doc = "0x210 - CAN AF ram access register" ]
    pub mask132: MASK,
    # [ doc = "0x214 - CAN AF ram access register" ]
    pub mask133: MASK,
    # [ doc = "0x218 - CAN AF ram access register" ]
    pub mask134: MASK,
    # [ doc = "0x21c - CAN AF ram access register" ]
    pub mask135: MASK,
    # [ doc = "0x220 - CAN AF ram access register" ]
    pub mask136: MASK,
    # [ doc = "0x224 - CAN AF ram access register" ]
    pub mask137: MASK,
    # [ doc = "0x228 - CAN AF ram access register" ]
    pub mask138: MASK,
    # [ doc = "0x22c - CAN AF ram access register" ]
    pub mask139: MASK,
    # [ doc = "0x230 - CAN AF ram access register" ]
    pub mask140: MASK,
    # [ doc = "0x234 - CAN AF ram access register" ]
    pub mask141: MASK,
    # [ doc = "0x238 - CAN AF ram access register" ]
    pub mask142: MASK,
    # [ doc = "0x23c - CAN AF ram access register" ]
    pub mask143: MASK,
    # [ doc = "0x240 - CAN AF ram access register" ]
    pub mask144: MASK,
    # [ doc = "0x244 - CAN AF ram access register" ]
    pub mask145: MASK,
    # [ doc = "0x248 - CAN AF ram access register" ]
    pub mask146: MASK,
    # [ doc = "0x24c - CAN AF ram access register" ]
    pub mask147: MASK,
    # [ doc = "0x250 - CAN AF ram access register" ]
    pub mask148: MASK,
    # [ doc = "0x254 - CAN AF ram access register" ]
    pub mask149: MASK,
    # [ doc = "0x258 - CAN AF ram access register" ]
    pub mask150: MASK,
    # [ doc = "0x25c - CAN AF ram access register" ]
    pub mask151: MASK,
    # [ doc = "0x260 - CAN AF ram access register" ]
    pub mask152: MASK,
    # [ doc = "0x264 - CAN AF ram access register" ]
    pub mask153: MASK,
    # [ doc = "0x268 - CAN AF ram access register" ]
    pub mask154: MASK,
    # [ doc = "0x26c - CAN AF ram access register" ]
    pub mask155: MASK,
    # [ doc = "0x270 - CAN AF ram access register" ]
    pub mask156: MASK,
    # [ doc = "0x274 - CAN AF ram access register" ]
    pub mask157: MASK,
    # [ doc = "0x278 - CAN AF ram access register" ]
    pub mask158: MASK,
    # [ doc = "0x27c - CAN AF ram access register" ]
    pub mask159: MASK,
    # [ doc = "0x280 - CAN AF ram access register" ]
    pub mask160: MASK,
    # [ doc = "0x284 - CAN AF ram access register" ]
    pub mask161: MASK,
    # [ doc = "0x288 - CAN AF ram access register" ]
    pub mask162: MASK,
    # [ doc = "0x28c - CAN AF ram access register" ]
    pub mask163: MASK,
    # [ doc = "0x290 - CAN AF ram access register" ]
    pub mask164: MASK,
    # [ doc = "0x294 - CAN AF ram access register" ]
    pub mask165: MASK,
    # [ doc = "0x298 - CAN AF ram access register" ]
    pub mask166: MASK,
    # [ doc = "0x29c - CAN AF ram access register" ]
    pub mask167: MASK,
    # [ doc = "0x2a0 - CAN AF ram access register" ]
    pub mask168: MASK,
    # [ doc = "0x2a4 - CAN AF ram access register" ]
    pub mask169: MASK,
    # [ doc = "0x2a8 - CAN AF ram access register" ]
    pub mask170: MASK,
    # [ doc = "0x2ac - CAN AF ram access register" ]
    pub mask171: MASK,
    # [ doc = "0x2b0 - CAN AF ram access register" ]
    pub mask172: MASK,
    # [ doc = "0x2b4 - CAN AF ram access register" ]
    pub mask173: MASK,
    # [ doc = "0x2b8 - CAN AF ram access register" ]
    pub mask174: MASK,
    # [ doc = "0x2bc - CAN AF ram access register" ]
    pub mask175: MASK,
    # [ doc = "0x2c0 - CAN AF ram access register" ]
    pub mask176: MASK,
    # [ doc = "0x2c4 - CAN AF ram access register" ]
    pub mask177: MASK,
    # [ doc = "0x2c8 - CAN AF ram access register" ]
    pub mask178: MASK,
    # [ doc = "0x2cc - CAN AF ram access register" ]
    pub mask179: MASK,
    # [ doc = "0x2d0 - CAN AF ram access register" ]
    pub mask180: MASK,
    # [ doc = "0x2d4 - CAN AF ram access register" ]
    pub mask181: MASK,
    # [ doc = "0x2d8 - CAN AF ram access register" ]
    pub mask182: MASK,
    # [ doc = "0x2dc - CAN AF ram access register" ]
    pub mask183: MASK,
    # [ doc = "0x2e0 - CAN AF ram access register" ]
    pub mask184: MASK,
    # [ doc = "0x2e4 - CAN AF ram access register" ]
    pub mask185: MASK,
    # [ doc = "0x2e8 - CAN AF ram access register" ]
    pub mask186: MASK,
    # [ doc = "0x2ec - CAN AF ram access register" ]
    pub mask187: MASK,
    # [ doc = "0x2f0 - CAN AF ram access register" ]
    pub mask188: MASK,
    # [ doc = "0x2f4 - CAN AF ram access register" ]
    pub mask189: MASK,
    # [ doc = "0x2f8 - CAN AF ram access register" ]
    pub mask190: MASK,
    # [ doc = "0x2fc - CAN AF ram access register" ]
    pub mask191: MASK,
    # [ doc = "0x300 - CAN AF ram access register" ]
    pub mask192: MASK,
    # [ doc = "0x304 - CAN AF ram access register" ]
    pub mask193: MASK,
    # [ doc = "0x308 - CAN AF ram access register" ]
    pub mask194: MASK,
    # [ doc = "0x30c - CAN AF ram access register" ]
    pub mask195: MASK,
    # [ doc = "0x310 - CAN AF ram access register" ]
    pub mask196: MASK,
    # [ doc = "0x314 - CAN AF ram access register" ]
    pub mask197: MASK,
    # [ doc = "0x318 - CAN AF ram access register" ]
    pub mask198: MASK,
    # [ doc = "0x31c - CAN AF ram access register" ]
    pub mask199: MASK,
    # [ doc = "0x320 - CAN AF ram access register" ]
    pub mask200: MASK,
    # [ doc = "0x324 - CAN AF ram access register" ]
    pub mask201: MASK,
    # [ doc = "0x328 - CAN AF ram access register" ]
    pub mask202: MASK,
    # [ doc = "0x32c - CAN AF ram access register" ]
    pub mask203: MASK,
    # [ doc = "0x330 - CAN AF ram access register" ]
    pub mask204: MASK,
    # [ doc = "0x334 - CAN AF ram access register" ]
    pub mask205: MASK,
    # [ doc = "0x338 - CAN AF ram access register" ]
    pub mask206: MASK,
    # [ doc = "0x33c - CAN AF ram access register" ]
    pub mask207: MASK,
    # [ doc = "0x340 - CAN AF ram access register" ]
    pub mask208: MASK,
    # [ doc = "0x344 - CAN AF ram access register" ]
    pub mask209: MASK,
    # [ doc = "0x348 - CAN AF ram access register" ]
    pub mask210: MASK,
    # [ doc = "0x34c - CAN AF ram access register" ]
    pub mask211: MASK,
    # [ doc = "0x350 - CAN AF ram access register" ]
    pub mask212: MASK,
    # [ doc = "0x354 - CAN AF ram access register" ]
    pub mask213: MASK,
    # [ doc = "0x358 - CAN AF ram access register" ]
    pub mask214: MASK,
    # [ doc = "0x35c - CAN AF ram access register" ]
    pub mask215: MASK,
    # [ doc = "0x360 - CAN AF ram access register" ]
    pub mask216: MASK,
    # [ doc = "0x364 - CAN AF ram access register" ]
    pub mask217: MASK,
    # [ doc = "0x368 - CAN AF ram access register" ]
    pub mask218: MASK,
    # [ doc = "0x36c - CAN AF ram access register" ]
    pub mask219: MASK,
    # [ doc = "0x370 - CAN AF ram access register" ]
    pub mask220: MASK,
    # [ doc = "0x374 - CAN AF ram access register" ]
    pub mask221: MASK,
    # [ doc = "0x378 - CAN AF ram access register" ]
    pub mask222: MASK,
    # [ doc = "0x37c - CAN AF ram access register" ]
    pub mask223: MASK,
    # [ doc = "0x380 - CAN AF ram access register" ]
    pub mask224: MASK,
    # [ doc = "0x384 - CAN AF ram access register" ]
    pub mask225: MASK,
    # [ doc = "0x388 - CAN AF ram access register" ]
    pub mask226: MASK,
    # [ doc = "0x38c - CAN AF ram access register" ]
    pub mask227: MASK,
    # [ doc = "0x390 - CAN AF ram access register" ]
    pub mask228: MASK,
    # [ doc = "0x394 - CAN AF ram access register" ]
    pub mask229: MASK,
    # [ doc = "0x398 - CAN AF ram access register" ]
    pub mask230: MASK,
    # [ doc = "0x39c - CAN AF ram access register" ]
    pub mask231: MASK,
    # [ doc = "0x3a0 - CAN AF ram access register" ]
    pub mask232: MASK,
    # [ doc = "0x3a4 - CAN AF ram access register" ]
    pub mask233: MASK,
    # [ doc = "0x3a8 - CAN AF ram access register" ]
    pub mask234: MASK,
    # [ doc = "0x3ac - CAN AF ram access register" ]
    pub mask235: MASK,
    # [ doc = "0x3b0 - CAN AF ram access register" ]
    pub mask236: MASK,
    # [ doc = "0x3b4 - CAN AF ram access register" ]
    pub mask237: MASK,
    # [ doc = "0x3b8 - CAN AF ram access register" ]
    pub mask238: MASK,
    # [ doc = "0x3bc - CAN AF ram access register" ]
    pub mask239: MASK,
    # [ doc = "0x3c0 - CAN AF ram access register" ]
    pub mask240: MASK,
    # [ doc = "0x3c4 - CAN AF ram access register" ]
    pub mask241: MASK,
    # [ doc = "0x3c8 - CAN AF ram access register" ]
    pub mask242: MASK,
    # [ doc = "0x3cc - CAN AF ram access register" ]
    pub mask243: MASK,
    # [ doc = "0x3d0 - CAN AF ram access register" ]
    pub mask244: MASK,
    # [ doc = "0x3d4 - CAN AF ram access register" ]
    pub mask245: MASK,
    # [ doc = "0x3d8 - CAN AF ram access register" ]
    pub mask246: MASK,
    # [ doc = "0x3dc - CAN AF ram access register" ]
    pub mask247: MASK,
    # [ doc = "0x3e0 - CAN AF ram access register" ]
    pub mask248: MASK,
    # [ doc = "0x3e4 - CAN AF ram access register" ]
    pub mask249: MASK,
    # [ doc = "0x3e8 - CAN AF ram access register" ]
    pub mask250: MASK,
    # [ doc = "0x3ec - CAN AF ram access register" ]
    pub mask251: MASK,
    # [ doc = "0x3f0 - CAN AF ram access register" ]
    pub mask252: MASK,
    # [ doc = "0x3f4 - CAN AF ram access register" ]
    pub mask253: MASK,
    # [ doc = "0x3f8 - CAN AF ram access register" ]
    pub mask254: MASK,
    # [ doc = "0x3fc - CAN AF ram access register" ]
    pub mask255: MASK,
    # [ doc = "0x400 - CAN AF ram access register" ]
    pub mask256: MASK,
    # [ doc = "0x404 - CAN AF ram access register" ]
    pub mask257: MASK,
    # [ doc = "0x408 - CAN AF ram access register" ]
    pub mask258: MASK,
    # [ doc = "0x40c - CAN AF ram access register" ]
    pub mask259: MASK,
    # [ doc = "0x410 - CAN AF ram access register" ]
    pub mask260: MASK,
    # [ doc = "0x414 - CAN AF ram access register" ]
    pub mask261: MASK,
    # [ doc = "0x418 - CAN AF ram access register" ]
    pub mask262: MASK,
    # [ doc = "0x41c - CAN AF ram access register" ]
    pub mask263: MASK,
    # [ doc = "0x420 - CAN AF ram access register" ]
    pub mask264: MASK,
    # [ doc = "0x424 - CAN AF ram access register" ]
    pub mask265: MASK,
    # [ doc = "0x428 - CAN AF ram access register" ]
    pub mask266: MASK,
    # [ doc = "0x42c - CAN AF ram access register" ]
    pub mask267: MASK,
    # [ doc = "0x430 - CAN AF ram access register" ]
    pub mask268: MASK,
    # [ doc = "0x434 - CAN AF ram access register" ]
    pub mask269: MASK,
    # [ doc = "0x438 - CAN AF ram access register" ]
    pub mask270: MASK,
    # [ doc = "0x43c - CAN AF ram access register" ]
    pub mask271: MASK,
    # [ doc = "0x440 - CAN AF ram access register" ]
    pub mask272: MASK,
    # [ doc = "0x444 - CAN AF ram access register" ]
    pub mask273: MASK,
    # [ doc = "0x448 - CAN AF ram access register" ]
    pub mask274: MASK,
    # [ doc = "0x44c - CAN AF ram access register" ]
    pub mask275: MASK,
    # [ doc = "0x450 - CAN AF ram access register" ]
    pub mask276: MASK,
    # [ doc = "0x454 - CAN AF ram access register" ]
    pub mask277: MASK,
    # [ doc = "0x458 - CAN AF ram access register" ]
    pub mask278: MASK,
    # [ doc = "0x45c - CAN AF ram access register" ]
    pub mask279: MASK,
    # [ doc = "0x460 - CAN AF ram access register" ]
    pub mask280: MASK,
    # [ doc = "0x464 - CAN AF ram access register" ]
    pub mask281: MASK,
    # [ doc = "0x468 - CAN AF ram access register" ]
    pub mask282: MASK,
    # [ doc = "0x46c - CAN AF ram access register" ]
    pub mask283: MASK,
    # [ doc = "0x470 - CAN AF ram access register" ]
    pub mask284: MASK,
    # [ doc = "0x474 - CAN AF ram access register" ]
    pub mask285: MASK,
    # [ doc = "0x478 - CAN AF ram access register" ]
    pub mask286: MASK,
    # [ doc = "0x47c - CAN AF ram access register" ]
    pub mask287: MASK,
    # [ doc = "0x480 - CAN AF ram access register" ]
    pub mask288: MASK,
    # [ doc = "0x484 - CAN AF ram access register" ]
    pub mask289: MASK,
    # [ doc = "0x488 - CAN AF ram access register" ]
    pub mask290: MASK,
    # [ doc = "0x48c - CAN AF ram access register" ]
    pub mask291: MASK,
    # [ doc = "0x490 - CAN AF ram access register" ]
    pub mask292: MASK,
    # [ doc = "0x494 - CAN AF ram access register" ]
    pub mask293: MASK,
    # [ doc = "0x498 - CAN AF ram access register" ]
    pub mask294: MASK,
    # [ doc = "0x49c - CAN AF ram access register" ]
    pub mask295: MASK,
    # [ doc = "0x4a0 - CAN AF ram access register" ]
    pub mask296: MASK,
    # [ doc = "0x4a4 - CAN AF ram access register" ]
    pub mask297: MASK,
    # [ doc = "0x4a8 - CAN AF ram access register" ]
    pub mask298: MASK,
    # [ doc = "0x4ac - CAN AF ram access register" ]
    pub mask299: MASK,
    # [ doc = "0x4b0 - CAN AF ram access register" ]
    pub mask300: MASK,
    # [ doc = "0x4b4 - CAN AF ram access register" ]
    pub mask301: MASK,
    # [ doc = "0x4b8 - CAN AF ram access register" ]
    pub mask302: MASK,
    # [ doc = "0x4bc - CAN AF ram access register" ]
    pub mask303: MASK,
    # [ doc = "0x4c0 - CAN AF ram access register" ]
    pub mask304: MASK,
    # [ doc = "0x4c4 - CAN AF ram access register" ]
    pub mask305: MASK,
    # [ doc = "0x4c8 - CAN AF ram access register" ]
    pub mask306: MASK,
    # [ doc = "0x4cc - CAN AF ram access register" ]
    pub mask307: MASK,
    # [ doc = "0x4d0 - CAN AF ram access register" ]
    pub mask308: MASK,
    # [ doc = "0x4d4 - CAN AF ram access register" ]
    pub mask309: MASK,
    # [ doc = "0x4d8 - CAN AF ram access register" ]
    pub mask310: MASK,
    # [ doc = "0x4dc - CAN AF ram access register" ]
    pub mask311: MASK,
    # [ doc = "0x4e0 - CAN AF ram access register" ]
    pub mask312: MASK,
    # [ doc = "0x4e4 - CAN AF ram access register" ]
    pub mask313: MASK,
    # [ doc = "0x4e8 - CAN AF ram access register" ]
    pub mask314: MASK,
    # [ doc = "0x4ec - CAN AF ram access register" ]
    pub mask315: MASK,
    # [ doc = "0x4f0 - CAN AF ram access register" ]
    pub mask316: MASK,
    # [ doc = "0x4f4 - CAN AF ram access register" ]
    pub mask317: MASK,
    # [ doc = "0x4f8 - CAN AF ram access register" ]
    pub mask318: MASK,
    # [ doc = "0x4fc - CAN AF ram access register" ]
    pub mask319: MASK,
    # [ doc = "0x500 - CAN AF ram access register" ]
    pub mask320: MASK,
    # [ doc = "0x504 - CAN AF ram access register" ]
    pub mask321: MASK,
    # [ doc = "0x508 - CAN AF ram access register" ]
    pub mask322: MASK,
    # [ doc = "0x50c - CAN AF ram access register" ]
    pub mask323: MASK,
    # [ doc = "0x510 - CAN AF ram access register" ]
    pub mask324: MASK,
    # [ doc = "0x514 - CAN AF ram access register" ]
    pub mask325: MASK,
    # [ doc = "0x518 - CAN AF ram access register" ]
    pub mask326: MASK,
    # [ doc = "0x51c - CAN AF ram access register" ]
    pub mask327: MASK,
    # [ doc = "0x520 - CAN AF ram access register" ]
    pub mask328: MASK,
    # [ doc = "0x524 - CAN AF ram access register" ]
    pub mask329: MASK,
    # [ doc = "0x528 - CAN AF ram access register" ]
    pub mask330: MASK,
    # [ doc = "0x52c - CAN AF ram access register" ]
    pub mask331: MASK,
    # [ doc = "0x530 - CAN AF ram access register" ]
    pub mask332: MASK,
    # [ doc = "0x534 - CAN AF ram access register" ]
    pub mask333: MASK,
    # [ doc = "0x538 - CAN AF ram access register" ]
    pub mask334: MASK,
    # [ doc = "0x53c - CAN AF ram access register" ]
    pub mask335: MASK,
    # [ doc = "0x540 - CAN AF ram access register" ]
    pub mask336: MASK,
    # [ doc = "0x544 - CAN AF ram access register" ]
    pub mask337: MASK,
    # [ doc = "0x548 - CAN AF ram access register" ]
    pub mask338: MASK,
    # [ doc = "0x54c - CAN AF ram access register" ]
    pub mask339: MASK,
    # [ doc = "0x550 - CAN AF ram access register" ]
    pub mask340: MASK,
    # [ doc = "0x554 - CAN AF ram access register" ]
    pub mask341: MASK,
    # [ doc = "0x558 - CAN AF ram access register" ]
    pub mask342: MASK,
    # [ doc = "0x55c - CAN AF ram access register" ]
    pub mask343: MASK,
    # [ doc = "0x560 - CAN AF ram access register" ]
    pub mask344: MASK,
    # [ doc = "0x564 - CAN AF ram access register" ]
    pub mask345: MASK,
    # [ doc = "0x568 - CAN AF ram access register" ]
    pub mask346: MASK,
    # [ doc = "0x56c - CAN AF ram access register" ]
    pub mask347: MASK,
    # [ doc = "0x570 - CAN AF ram access register" ]
    pub mask348: MASK,
    # [ doc = "0x574 - CAN AF ram access register" ]
    pub mask349: MASK,
    # [ doc = "0x578 - CAN AF ram access register" ]
    pub mask350: MASK,
    # [ doc = "0x57c - CAN AF ram access register" ]
    pub mask351: MASK,
    # [ doc = "0x580 - CAN AF ram access register" ]
    pub mask352: MASK,
    # [ doc = "0x584 - CAN AF ram access register" ]
    pub mask353: MASK,
    # [ doc = "0x588 - CAN AF ram access register" ]
    pub mask354: MASK,
    # [ doc = "0x58c - CAN AF ram access register" ]
    pub mask355: MASK,
    # [ doc = "0x590 - CAN AF ram access register" ]
    pub mask356: MASK,
    # [ doc = "0x594 - CAN AF ram access register" ]
    pub mask357: MASK,
    # [ doc = "0x598 - CAN AF ram access register" ]
    pub mask358: MASK,
    # [ doc = "0x59c - CAN AF ram access register" ]
    pub mask359: MASK,
    # [ doc = "0x5a0 - CAN AF ram access register" ]
    pub mask360: MASK,
    # [ doc = "0x5a4 - CAN AF ram access register" ]
    pub mask361: MASK,
    # [ doc = "0x5a8 - CAN AF ram access register" ]
    pub mask362: MASK,
    # [ doc = "0x5ac - CAN AF ram access register" ]
    pub mask363: MASK,
    # [ doc = "0x5b0 - CAN AF ram access register" ]
    pub mask364: MASK,
    # [ doc = "0x5b4 - CAN AF ram access register" ]
    pub mask365: MASK,
    # [ doc = "0x5b8 - CAN AF ram access register" ]
    pub mask366: MASK,
    # [ doc = "0x5bc - CAN AF ram access register" ]
    pub mask367: MASK,
    # [ doc = "0x5c0 - CAN AF ram access register" ]
    pub mask368: MASK,
    # [ doc = "0x5c4 - CAN AF ram access register" ]
    pub mask369: MASK,
    # [ doc = "0x5c8 - CAN AF ram access register" ]
    pub mask370: MASK,
    # [ doc = "0x5cc - CAN AF ram access register" ]
    pub mask371: MASK,
    # [ doc = "0x5d0 - CAN AF ram access register" ]
    pub mask372: MASK,
    # [ doc = "0x5d4 - CAN AF ram access register" ]
    pub mask373: MASK,
    # [ doc = "0x5d8 - CAN AF ram access register" ]
    pub mask374: MASK,
    # [ doc = "0x5dc - CAN AF ram access register" ]
    pub mask375: MASK,
    # [ doc = "0x5e0 - CAN AF ram access register" ]
    pub mask376: MASK,
    # [ doc = "0x5e4 - CAN AF ram access register" ]
    pub mask377: MASK,
    # [ doc = "0x5e8 - CAN AF ram access register" ]
    pub mask378: MASK,
    # [ doc = "0x5ec - CAN AF ram access register" ]
    pub mask379: MASK,
    # [ doc = "0x5f0 - CAN AF ram access register" ]
    pub mask380: MASK,
    # [ doc = "0x5f4 - CAN AF ram access register" ]
    pub mask381: MASK,
    # [ doc = "0x5f8 - CAN AF ram access register" ]
    pub mask382: MASK,
    # [ doc = "0x5fc - CAN AF ram access register" ]
    pub mask383: MASK,
    # [ doc = "0x600 - CAN AF ram access register" ]
    pub mask384: MASK,
    # [ doc = "0x604 - CAN AF ram access register" ]
    pub mask385: MASK,
    # [ doc = "0x608 - CAN AF ram access register" ]
    pub mask386: MASK,
    # [ doc = "0x60c - CAN AF ram access register" ]
    pub mask387: MASK,
    # [ doc = "0x610 - CAN AF ram access register" ]
    pub mask388: MASK,
    # [ doc = "0x614 - CAN AF ram access register" ]
    pub mask389: MASK,
    # [ doc = "0x618 - CAN AF ram access register" ]
    pub mask390: MASK,
    # [ doc = "0x61c - CAN AF ram access register" ]
    pub mask391: MASK,
    # [ doc = "0x620 - CAN AF ram access register" ]
    pub mask392: MASK,
    # [ doc = "0x624 - CAN AF ram access register" ]
    pub mask393: MASK,
    # [ doc = "0x628 - CAN AF ram access register" ]
    pub mask394: MASK,
    # [ doc = "0x62c - CAN AF ram access register" ]
    pub mask395: MASK,
    # [ doc = "0x630 - CAN AF ram access register" ]
    pub mask396: MASK,
    # [ doc = "0x634 - CAN AF ram access register" ]
    pub mask397: MASK,
    # [ doc = "0x638 - CAN AF ram access register" ]
    pub mask398: MASK,
    # [ doc = "0x63c - CAN AF ram access register" ]
    pub mask399: MASK,
    # [ doc = "0x640 - CAN AF ram access register" ]
    pub mask400: MASK,
    # [ doc = "0x644 - CAN AF ram access register" ]
    pub mask401: MASK,
    # [ doc = "0x648 - CAN AF ram access register" ]
    pub mask402: MASK,
    # [ doc = "0x64c - CAN AF ram access register" ]
    pub mask403: MASK,
    # [ doc = "0x650 - CAN AF ram access register" ]
    pub mask404: MASK,
    # [ doc = "0x654 - CAN AF ram access register" ]
    pub mask405: MASK,
    # [ doc = "0x658 - CAN AF ram access register" ]
    pub mask406: MASK,
    # [ doc = "0x65c - CAN AF ram access register" ]
    pub mask407: MASK,
    # [ doc = "0x660 - CAN AF ram access register" ]
    pub mask408: MASK,
    # [ doc = "0x664 - CAN AF ram access register" ]
    pub mask409: MASK,
    # [ doc = "0x668 - CAN AF ram access register" ]
    pub mask410: MASK,
    # [ doc = "0x66c - CAN AF ram access register" ]
    pub mask411: MASK,
    # [ doc = "0x670 - CAN AF ram access register" ]
    pub mask412: MASK,
    # [ doc = "0x674 - CAN AF ram access register" ]
    pub mask413: MASK,
    # [ doc = "0x678 - CAN AF ram access register" ]
    pub mask414: MASK,
    # [ doc = "0x67c - CAN AF ram access register" ]
    pub mask415: MASK,
    # [ doc = "0x680 - CAN AF ram access register" ]
    pub mask416: MASK,
    # [ doc = "0x684 - CAN AF ram access register" ]
    pub mask417: MASK,
    # [ doc = "0x688 - CAN AF ram access register" ]
    pub mask418: MASK,
    # [ doc = "0x68c - CAN AF ram access register" ]
    pub mask419: MASK,
    # [ doc = "0x690 - CAN AF ram access register" ]
    pub mask420: MASK,
    # [ doc = "0x694 - CAN AF ram access register" ]
    pub mask421: MASK,
    # [ doc = "0x698 - CAN AF ram access register" ]
    pub mask422: MASK,
    # [ doc = "0x69c - CAN AF ram access register" ]
    pub mask423: MASK,
    # [ doc = "0x6a0 - CAN AF ram access register" ]
    pub mask424: MASK,
    # [ doc = "0x6a4 - CAN AF ram access register" ]
    pub mask425: MASK,
    # [ doc = "0x6a8 - CAN AF ram access register" ]
    pub mask426: MASK,
    # [ doc = "0x6ac - CAN AF ram access register" ]
    pub mask427: MASK,
    # [ doc = "0x6b0 - CAN AF ram access register" ]
    pub mask428: MASK,
    # [ doc = "0x6b4 - CAN AF ram access register" ]
    pub mask429: MASK,
    # [ doc = "0x6b8 - CAN AF ram access register" ]
    pub mask430: MASK,
    # [ doc = "0x6bc - CAN AF ram access register" ]
    pub mask431: MASK,
    # [ doc = "0x6c0 - CAN AF ram access register" ]
    pub mask432: MASK,
    # [ doc = "0x6c4 - CAN AF ram access register" ]
    pub mask433: MASK,
    # [ doc = "0x6c8 - CAN AF ram access register" ]
    pub mask434: MASK,
    # [ doc = "0x6cc - CAN AF ram access register" ]
    pub mask435: MASK,
    # [ doc = "0x6d0 - CAN AF ram access register" ]
    pub mask436: MASK,
    # [ doc = "0x6d4 - CAN AF ram access register" ]
    pub mask437: MASK,
    # [ doc = "0x6d8 - CAN AF ram access register" ]
    pub mask438: MASK,
    # [ doc = "0x6dc - CAN AF ram access register" ]
    pub mask439: MASK,
    # [ doc = "0x6e0 - CAN AF ram access register" ]
    pub mask440: MASK,
    # [ doc = "0x6e4 - CAN AF ram access register" ]
    pub mask441: MASK,
    # [ doc = "0x6e8 - CAN AF ram access register" ]
    pub mask442: MASK,
    # [ doc = "0x6ec - CAN AF ram access register" ]
    pub mask443: MASK,
    # [ doc = "0x6f0 - CAN AF ram access register" ]
    pub mask444: MASK,
    # [ doc = "0x6f4 - CAN AF ram access register" ]
    pub mask445: MASK,
    # [ doc = "0x6f8 - CAN AF ram access register" ]
    pub mask446: MASK,
    # [ doc = "0x6fc - CAN AF ram access register" ]
    pub mask447: MASK,
    # [ doc = "0x700 - CAN AF ram access register" ]
    pub mask448: MASK,
    # [ doc = "0x704 - CAN AF ram access register" ]
    pub mask449: MASK,
    # [ doc = "0x708 - CAN AF ram access register" ]
    pub mask450: MASK,
    # [ doc = "0x70c - CAN AF ram access register" ]
    pub mask451: MASK,
    # [ doc = "0x710 - CAN AF ram access register" ]
    pub mask452: MASK,
    # [ doc = "0x714 - CAN AF ram access register" ]
    pub mask453: MASK,
    # [ doc = "0x718 - CAN AF ram access register" ]
    pub mask454: MASK,
    # [ doc = "0x71c - CAN AF ram access register" ]
    pub mask455: MASK,
    # [ doc = "0x720 - CAN AF ram access register" ]
    pub mask456: MASK,
    # [ doc = "0x724 - CAN AF ram access register" ]
    pub mask457: MASK,
    # [ doc = "0x728 - CAN AF ram access register" ]
    pub mask458: MASK,
    # [ doc = "0x72c - CAN AF ram access register" ]
    pub mask459: MASK,
    # [ doc = "0x730 - CAN AF ram access register" ]
    pub mask460: MASK,
    # [ doc = "0x734 - CAN AF ram access register" ]
    pub mask461: MASK,
    # [ doc = "0x738 - CAN AF ram access register" ]
    pub mask462: MASK,
    # [ doc = "0x73c - CAN AF ram access register" ]
    pub mask463: MASK,
    # [ doc = "0x740 - CAN AF ram access register" ]
    pub mask464: MASK,
    # [ doc = "0x744 - CAN AF ram access register" ]
    pub mask465: MASK,
    # [ doc = "0x748 - CAN AF ram access register" ]
    pub mask466: MASK,
    # [ doc = "0x74c - CAN AF ram access register" ]
    pub mask467: MASK,
    # [ doc = "0x750 - CAN AF ram access register" ]
    pub mask468: MASK,
    # [ doc = "0x754 - CAN AF ram access register" ]
    pub mask469: MASK,
    # [ doc = "0x758 - CAN AF ram access register" ]
    pub mask470: MASK,
    # [ doc = "0x75c - CAN AF ram access register" ]
    pub mask471: MASK,
    # [ doc = "0x760 - CAN AF ram access register" ]
    pub mask472: MASK,
    # [ doc = "0x764 - CAN AF ram access register" ]
    pub mask473: MASK,
    # [ doc = "0x768 - CAN AF ram access register" ]
    pub mask474: MASK,
    # [ doc = "0x76c - CAN AF ram access register" ]
    pub mask475: MASK,
    # [ doc = "0x770 - CAN AF ram access register" ]
    pub mask476: MASK,
    # [ doc = "0x774 - CAN AF ram access register" ]
    pub mask477: MASK,
    # [ doc = "0x778 - CAN AF ram access register" ]
    pub mask478: MASK,
    # [ doc = "0x77c - CAN AF ram access register" ]
    pub mask479: MASK,
    # [ doc = "0x780 - CAN AF ram access register" ]
    pub mask480: MASK,
    # [ doc = "0x784 - CAN AF ram access register" ]
    pub mask481: MASK,
    # [ doc = "0x788 - CAN AF ram access register" ]
    pub mask482: MASK,
    # [ doc = "0x78c - CAN AF ram access register" ]
    pub mask483: MASK,
    # [ doc = "0x790 - CAN AF ram access register" ]
    pub mask484: MASK,
    # [ doc = "0x794 - CAN AF ram access register" ]
    pub mask485: MASK,
    # [ doc = "0x798 - CAN AF ram access register" ]
    pub mask486: MASK,
    # [ doc = "0x79c - CAN AF ram access register" ]
    pub mask487: MASK,
    # [ doc = "0x7a0 - CAN AF ram access register" ]
    pub mask488: MASK,
    # [ doc = "0x7a4 - CAN AF ram access register" ]
    pub mask489: MASK,
    # [ doc = "0x7a8 - CAN AF ram access register" ]
    pub mask490: MASK,
    # [ doc = "0x7ac - CAN AF ram access register" ]
    pub mask491: MASK,
    # [ doc = "0x7b0 - CAN AF ram access register" ]
    pub mask492: MASK,
    # [ doc = "0x7b4 - CAN AF ram access register" ]
    pub mask493: MASK,
    # [ doc = "0x7b8 - CAN AF ram access register" ]
    pub mask494: MASK,
    # [ doc = "0x7bc - CAN AF ram access register" ]
    pub mask495: MASK,
    # [ doc = "0x7c0 - CAN AF ram access register" ]
    pub mask496: MASK,
    # [ doc = "0x7c4 - CAN AF ram access register" ]
    pub mask497: MASK,
    # [ doc = "0x7c8 - CAN AF ram access register" ]
    pub mask498: MASK,
    # [ doc = "0x7cc - CAN AF ram access register" ]
    pub mask499: MASK,
    # [ doc = "0x7d0 - CAN AF ram access register" ]
    pub mask500: MASK,
    # [ doc = "0x7d4 - CAN AF ram access register" ]
    pub mask501: MASK,
    # [ doc = "0x7d8 - CAN AF ram access register" ]
    pub mask502: MASK,
    # [ doc = "0x7dc - CAN AF ram access register" ]
    pub mask503: MASK,
    # [ doc = "0x7e0 - CAN AF ram access register" ]
    pub mask504: MASK,
    # [ doc = "0x7e4 - CAN AF ram access register" ]
    pub mask505: MASK,
    # [ doc = "0x7e8 - CAN AF ram access register" ]
    pub mask506: MASK,
    # [ doc = "0x7ec - CAN AF ram access register" ]
    pub mask507: MASK,
    # [ doc = "0x7f0 - CAN AF ram access register" ]
    pub mask508: MASK,
    # [ doc = "0x7f4 - CAN AF ram access register" ]
    pub mask509: MASK,
    # [ doc = "0x7f8 - CAN AF ram access register" ]
    pub mask510: MASK,
    # [ doc = "0x7fc - CAN AF ram access register" ]
    pub mask511: MASK,
}
# [ doc = "CAN AF ram access register" ]
pub struct MASK {
    register: VolatileCell<u32>,
}
# [ doc = "CAN AF ram access register" ]
pub mod mask {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::MASK {
        # [ doc = r" Modifies the contents of the register" ]
        # [ inline ( always ) ]
        pub fn modify<F>(&self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.get();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        # [ inline ( always ) ]
        pub fn read(&self) -> R {
            R { bits: self.register.get() }
        }
        # [ doc = r" Writes to the register" ]
        # [ inline ( always ) ]
        pub fn write<F>(&self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.set(w.bits);
        }
        # [ doc = r" Writes the reset value to the register" ]
        # [ inline ( always ) ]
        pub fn reset(&self) {
            self.write(|w| w)
        }
    }
    # [ doc = r" Value of the field" ]
    pub struct MASKR {
        bits: u32,
    }
    impl MASKR {
        # [ doc = r" Value of the field as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MASKW<'a> {
        w: &'a mut W,
    }
    impl<'a> _MASKW<'a> {
        # [ doc = r" Writes raw bits to the field" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(self, value: u32) -> &'a mut W {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            self.w.bits &= !((MASK as u32) << OFFSET);
            self.w.bits |= ((value & MASK) as u32) << OFFSET;
            self.w
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        # [ inline ( always ) ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        # [ doc = "Bits 0:31 - CAN AF RAM mask" ]
        # [ inline ( always ) ]
        pub fn mask(&self) -> MASKR {
            let bits = {
                const MASK: u32 = 4294967295;
                const OFFSET: u8 = 0;
                ((self.bits >> OFFSET) & MASK as u32) as u32
            };
            MASKR { bits }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        # [ inline ( always ) ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw bits to the register" ]
        # [ inline ( always ) ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 0:31 - CAN AF RAM mask" ]
        # [ inline ( always ) ]
        pub fn mask(&mut self) -> _MASKW {
            _MASKW { w: self }
        }
    }
}
# [ doc = "CAN acceptance filter RAM" ]
pub struct CANAFRAM {
    register_block: RegisterBlock,
}
impl Deref for CANAFRAM {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        &self.register_block
    }
}
