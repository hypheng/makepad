// stripped mac core foundation + metal layer only whats needed

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

pub use {
    std::{
        ffi::c_void,
        os::raw::c_ulong,
        ptr::NonNull,
    },
    
    crate::{
        os::apple::apple_util::four_char_as_u32,
        os::apple::apple_sys::*,
        makepad_objc_sys::{
            runtime::{Class, Object, Protocol, Sel, BOOL, YES, NO},
            declare::ClassDecl,
            msg_send,
            sel,
            class,
            sel_impl,
            Encode,
            Encoding
        },
    }
};


// CORE AUDIO


pub const kAudioUnitManufacturer_Apple: u32 = 1634758764;

#[repr(C)] pub struct OpaqueAudioComponent([u8; 0]);
pub type CAudioComponent = *mut OpaqueAudioComponent;

#[repr(C)] pub struct ComponentInstanceRecord([u8; 0]);
pub type CAudioComponentInstance = *mut ComponentInstanceRecord;
pub type CAudioUnit = CAudioComponentInstance;

pub type OSStatus = i32;

#[repr(C)]
#[derive(Debug)]
pub struct CAudioStreamBasicDescription {
    pub mSampleRate: f64,
    pub mFormatID: AudioFormatId,
    pub mFormatFlags: u32,
    pub mBytesPerPacket: u32,
    pub mFramesPerPacket: u32,
    pub mBytesPerFrame: u32,
    pub mChannelsPerFrame: u32,
    pub mBitsPerChannel: u32,
    pub mReserved: u32,
}


#[repr(C)]
pub struct AudioChannelLayout {
    pub mChannelLayoutTag: AudioLayoutChannelTag,
    pub mChannelBitmap: u32,
    pub mNumberChannelDescriptions: u32,
    pub mChannelDescriptions: [AudioChannelDescription; 2]
}

/*
let channel_layout = AudioChannelLayout {
    mChannelLayoutTag: AudioLayoutChannelTag::Stereo,
    mChannelBitmap: 0,
    mNumberChannelDescriptions: 2,
    mChannelDescriptions: [
        AudioChannelDescription {
            mChannelLabel: AudioChannelLabel::Left,
            mChannelFlags: 0,
            mCoordinates: [0f32; 3]
        },
        AudioChannelDescription {
            mChannelLabel: AudioChannelLabel::Right,
            mChannelFlags: 0,
            mCoordinates: [0f32; 3]
        },
    ]
};
let av_channel_layout: ObjcId = msg_send![class!(AVAudioChannelLayout), alloc];
let () = msg_send![av_channel_layout, initWithLayout: &channel_layout];
*/

#[repr(u32)]
pub enum AudioLayoutChannelTag {
    Stereo = (101 << 16) | 2,
}

#[repr(C)]
pub struct AudioChannelDescription {
    pub mChannelLabel: AudioChannelLabel,
    pub mChannelFlags: u32,
    pub mCoordinates: [f32; 3],
}

#[repr(u32)]
pub enum AudioChannelLabel {
    Left = 1,
    Right = 2,
}

#[derive(Debug)]
#[repr(u32)]
pub enum AudioFormatId {
    LinearPCM = 1819304813,
    AC3 = 1633889587,
    F60958AC3 = 1667326771,
    AppleIMA4 = 1768775988,
    MPEG4AAC = 1633772320,
    MPEG4CELP = 1667591280,
    MPEG4HVXC = 1752594531,
    MPEG4TwinVQ = 1953986161,
    MACE3 = 1296122675,
    MACE6 = 1296122678,
    ULaw = 1970037111,
    ALaw = 1634492791,
    QDesign = 1363430723,
    QDesign2 = 1363430706,
    QUALCOMM = 1365470320,
    MPEGLayer1 = 778924081,
    MPEGLayer2 = 778924082,
    MPEGLayer3 = 778924083,
    TimeCode = 1953066341,
    MIDIStream = 1835623529,
    ParameterValueStream = 1634760307,
    AppleLossless = 1634492771,
    MPEG4AAC_HE = 1633772392,
    MPEG4AAC_LD = 1633772396,
    MPEG4AAC_ELD = 1633772389,
    MPEG4AAC_ELD_SBR = 1633772390,
    MPEG4AAC_ELD_V2 = 1633772391,
    MPEG4AAC_HE_V2 = 1633772400,
    MPEG4AAC_Spatial = 1633772403,
    AMR = 1935764850,
    AMR_WB = 1935767394,
    Audible = 1096107074,
    iLBC = 1768710755,
    DVIIntelIMA = 1836253201,
    MicrosoftGSM = 1836253233,
    AES3 = 1634038579,
}
/*
struct F60958AC3Flags;
impl F60958AC3Flags {
    const IS_FLOAT: u32 = 1;
    const IS_BIG_ENDIAN: u32 = 2;
    const IS_SIGNED_INTEGER: u32 = 4;
    const IS_PACKED: u32 = 8;
    const IS_ALIGNED_HIGH: u32 = 16;
    const IS_NON_INTERLEAVED: u32 = 32;
    const IS_NON_MIXABLE: u32 = 64;
}
*/

#[repr(u32)]
pub enum LinearPcmFlags {
    IS_FLOAT = 1,
    IS_BIG_ENDIAN = 2,
    IS_SIGNED_INTEGER = 4,
    IS_PACKED = 8,
    IS_ALIGNED_HIGH = 16,
    IS_NON_INTERLEAVED = 32,
    IS_NON_MIXABLE = 64,
    FLAGS_SAMPLE_FRACTION_SHIFT = 7,
    FLAGS_SAMPLE_FRACTION_MASK = 8064,
}
/*
pub struct AppleLosslessFlags;
impl AppleLosslessFlags {
    const BIT_16_SOURCE_DATA: u32 = 1;
    const BIT_20_SOURCE_DATA: u32 = 2;
    const BIT_24_SOURCE_DATA: u32 = 3;
    const BIT_32_SOURCE_DATA: u32 = 4;
}
*/
#[repr(u32)]
pub enum Mpeg4ObjectId {
    AAC_Main = 1,
    AAC_LC = 2,
    AAC_SSR = 3,
    AAC_LTP = 4,
    AAC_SBR = 5,
    AAC_Scalable = 6,
    TwinVQ = 7,
    CELP = 8,
    HVXC = 9,
}
/*
pub struct AudioTimeStampFlags;
impl AudioTimeStampFlags {
    const SAMPLE_TIME_VALID: u32 = 1;
    const HOST_TIME_VALID: u32 = 2;
    const RATE_SCALAR_VALID: u32 = 4;
    const WORLD_CLOCK_TIME_VALID: u32 = 8;
    const SMPTE_TIME_VALID: u32 = 16;
}
*/
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(C)]
pub struct AudioComponentDescription {
    pub componentType: AudioUnitType,
    pub componentSubType: AudioUnitSubType,
    pub componentManufacturer: u32,
    pub componentFlags: u32,
    pub componentFlagsMask: u32,
}

impl AudioComponentDescription {
    pub fn new_apple(ty: AudioUnitType, sub: AudioUnitSubType) -> Self {
        Self {
            componentType: ty,
            componentSubType: sub,
            componentManufacturer: kAudioUnitManufacturer_Apple,
            componentFlags: 0,
            componentFlagsMask: 0,
        }
    }
    pub fn new_all_manufacturers(ty: AudioUnitType, sub: AudioUnitSubType) -> Self {
        Self {
            componentType: ty,
            componentSubType: sub,
            componentManufacturer: 0,
            componentFlags: 0,
            componentFlagsMask: 0,
        }
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct SMPTETime {
    pub mSubframes: i16,
    pub mSubframeDivisor: i16,
    pub mCounter: u32,
    pub mType: u32,
    pub mFlags: u32,
    pub mHours: i16,
    pub mMinutes: i16,
    pub mSeconds: i16,
    pub mFrames: i16,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct _AudioBuffer {
    pub mNumberChannels: u32,
    pub mDataByteSize: u32,
    pub mData: *mut ::std::os::raw::c_void,
}

pub const MAX_AUDIO_BUFFERS: usize = 8;
#[repr(C)]

#[derive(Debug)]
pub struct AudioBufferList {
    pub mNumberBuffers: u32,
    pub mBuffers: [_AudioBuffer; MAX_AUDIO_BUFFERS],
}

#[derive(Debug)]
#[repr(C)]
pub struct AudioTimeStamp {
    pub mSampleTime: f64,
    pub mHostTime: u64,
    pub mRateScalar: f64,
    pub mWordClockTime: u64,
    pub mSMPTETime: SMPTETime,
    pub mFlags: u32,
    pub mReserved: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum AudioUnitType {
    Undefined = 0,
    
    IO = 1635086197,
    MusicDevice = 1635085685,
    MusicEffect = 1635085670,
    FormatConverter = 1635083875,
    Effect = 1635083896,
    Mixer = 1635085688,
    Panner = 1635086446,
    Generator = 1635084142,
    OfflineEffect = 1635086188,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum AudioUnitSubType {
    Undefined = 0,
    
    PeakLimiter = 1819112562,
    DynamicsProcessor = 1684237680,
    LowPassFilter = 1819304307,
    HighPassFilter = 1752195443,
    BandPassFilter = 1651532147,
    HighShelfFilter = 1752393830,
    LowShelfFilter = 1819502694,
    ParametricEQ = 1886217585,
    Distortion = 1684632436,
    Delay = 1684368505,
    SampleDelay = 1935961209,
    GraphicEQ = 1735550321,
    MultiBandCompressor = 1835232624,
    MatrixReverb = 1836213622,
    Pitch = 1953329268,
    AUFilter = 1718185076,
    NetSend = 1853058660,
    RogerBeep = 1919903602,
    NBandEQ = 1851942257,
    
    //pub enum FormatConverterType
    AUConverter = 1668247158,
    NewTimePitch = 1853191280,
    //TimePitch = 1953329268,
    DeferredRenderer = 1684366962,
    Splitter = 1936747636,
    Merger = 1835364967,
    Varispeed = 1986097769,
    AUiPodTimeOther = 1768977519,
    
    //pub enum MixerType
    MultiChannelMixer = 1835232632,
    StereoMixer = 1936554098,
    Mixer3D = 862219640,
    MatrixMixer = 1836608888,
    
    //pub enum GeneratorType {
    ScheduledSoundPlayer = 1936945260,
    AudioFilePlayer = 1634103404,
    
    //pub enum MusicDeviceType {
    DLSSynth = 1684828960,
    Sampler = 1935764848,
    
    //pub enum IOType {
    GenericOutput = 1734700658,
    HalOutput = 1634230636,
    DefaultOutput = 1684366880,
    SystemOutput = 1937339168,
    VoiceProcessingIO = 1987078511,
    RemoteIO = 1919512419,
}

pub const kAudioComponentInstantiation_LoadInProcess: u32 = 2;
pub const kAudioComponentInstantiation_LoadOutOfProcess: u32 = 1;


pub type ItemCount = u64;
pub type MIDIObjectRef = u32;
pub type MIDIClientRef = MIDIObjectRef;
pub type MIDIPortRef = MIDIObjectRef;
pub type MIDIEndpointRef = MIDIObjectRef;
pub type MIDIProtocolID = i32;
pub type MIDITimeStamp = u64;
pub const kMIDIProtocol_1_0: i32 = 1;
pub const kMIDIProtocol_2_0: i32 = 2;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct MIDINotification {
    pub messageID: i32,
    pub messageSize: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MIDIEventList {
    pub protocol: MIDIProtocolID,
    pub numPackets: u32,
    pub packet: [MIDIEventPacket; 1usize],
}

#[repr(C, packed(4))]
#[derive(Copy, Clone)]
pub struct MIDIEventPacket {
    pub timeStamp: MIDITimeStamp,
    pub wordCount: u32,
    pub words: [u32; 64usize],
}

#[repr(u32)]
pub enum AudioObjectPropertySelector {
    Devices = four_char_as_u32("dev#"),
    DeviceNameCFString = four_char_as_u32("lnam"),
    StreamConfiguration = four_char_as_u32("slay"),
    DefaultInputDevice = four_char_as_u32("dIn "),
    DefaultOutputDevice = four_char_as_u32("dOut"),
    DeviceIsAlive = four_char_as_u32("livn"),
}

#[repr(u32)]
pub enum AudioObjectPropertyScope {
    Global = four_char_as_u32("glob"),
    Output = four_char_as_u32("outp"),
    Input = four_char_as_u32("inpt")}

#[repr(u32)]
pub enum AudioObjectPropertyElement {
    Master = 0
}

#[repr(C)]
pub struct AudioObjectPropertyAddress {
    pub mSelector: AudioObjectPropertySelector,
    pub mScope: AudioObjectPropertyScope,
    pub mElement: AudioObjectPropertyElement
}

pub const kAudioObjectSystemObject: AudioDeviceID = 1;
pub type AudioObjectID = u32;
pub type AudioDeviceID = u32;

#[link(name = "CoreMidi", kind = "framework")]
extern "C" {
    pub static kMIDIPropertyManufacturer: CFStringRef;
    pub static kMIDIPropertyDisplayName: CFStringRef;
    pub static kMIDIPropertyUniqueID: CFStringRef;
    
    pub fn MIDIGetNumberOfSources() -> ItemCount;
    pub fn MIDIGetSource(sourceIndex0: ItemCount) -> MIDIEndpointRef;
    
    pub fn MIDIGetNumberOfDestinations() -> ItemCount;
    pub fn MIDIGetDestination(sourceIndex0: ItemCount) -> MIDIEndpointRef;
    
    pub fn MIDISendEventList(
        port: MIDIPortRef,
        dest: MIDIEndpointRef,
        evtlist: *const MIDIEventList,
    ) -> OSStatus;
    
    pub fn MIDIClientCreateWithBlock(
        name: CFStringRef,
        outClient: *mut MIDIClientRef,
        notifyBlock: ObjcId,
    ) -> OSStatus;
    
    pub fn MIDIInputPortCreateWithProtocol(
        client: MIDIClientRef,
        portName: CFStringRef,
        protocol: MIDIProtocolID,
        outPort: *mut MIDIPortRef,
        receiveBlock: ObjcId,
    ) -> OSStatus;
    
    pub fn MIDIOutputPortCreate(
        client: MIDIClientRef,
        portName: CFStringRef,
        outPort: *mut MIDIPortRef,
    ) -> OSStatus;
    
    pub fn MIDIObjectGetStringProperty(
        obj: MIDIObjectRef,
        propertyID: CFStringRef,
        str_: *mut CFStringRef,
    ) -> OSStatus;
    
    pub fn MIDIObjectGetIntegerProperty(
        obj: MIDIObjectRef,
        propertyID: CFStringRef,
        outValue: *mut i32,
    ) -> OSStatus;
    
    pub fn MIDIPortConnectSource(
        port: MIDIPortRef,
        source: MIDIEndpointRef,
        connRefCon: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
    
    pub fn MIDIPortDisconnectSource(
        port: MIDIPortRef,
        source: MIDIEndpointRef,
    ) -> OSStatus;
    
}

pub type AudioObjectPropertyListenerProc = Option<
unsafe extern "system" fn(
    inObjectID: AudioObjectID,
    inNumberAddresses: u32,
    inAddresses: *const AudioObjectPropertyAddress,
    inClientData: *mut ()
) -> OSStatus>;

#[link(name = "CoreAudio", kind = "framework")]
extern "C" {
    pub fn AudioObjectGetPropertyDataSize(
        inObjectId: AudioObjectID,
        inAddress: *const AudioObjectPropertyAddress,
        inQualifierDataSize: u32,
        inQualifierData: *const (),
        outDataSize: *mut u32
    ) -> OSStatus;
    
    pub fn AudioObjectGetPropertyData(
        inObjectId: AudioObjectID,
        inAddress: *const AudioObjectPropertyAddress,
        inQualifierDataSize: u32,
        inQualifierData: *const (),
        ioDataSize: *mut u32,
        outData: *mut ()
    ) -> OSStatus;
    
    pub fn AudioObjectAddPropertyListener(
        inObjectId: AudioObjectID,
        inAddress: *const AudioObjectPropertyAddress,
        inListener: AudioObjectPropertyListenerProc,
        inClientData: *mut ()
    ) -> OSStatus;
    
}
