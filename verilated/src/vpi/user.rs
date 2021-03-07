#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::{c_char, c_double, c_float, c_int};

/*******************************************************************************
 * vpi_user.h
 *
 * IEEE Std 1800-2017 Programming Language Interface (PLI)
 *
 * This file contains the constant definitions, structure definitions, and
 * routine declarations used by the SystemVerilog Verification Procedural
 * Interface (VPI) access routines.
 *
 ******************************************************************************/

/*******************************************************************************
 * NOTE: the constant values 1 through 299 are reserved for use in this
 * vpi_user.h file.
 ******************************************************************************/

/* Sized variables */
pub type PLI_INT64 = i64;
pub type PLI_UINT64 = u64;

pub type PLI_INT32 = i32;
pub type PLI_UINT32 = u32;
pub type PLI_INT16 = i16;
pub type PLI_UINT16 = u16;
pub type PLI_BYTE8 = c_char;
pub type PLI_UBYTE8 = u8;

/********************************** TYPEDEFS **********************************/

pub type vpiHandle = *mut PLI_UINT32;

/******************************** OBJECT TYPES ********************************/

pub const vpiAlways: i32 = 1; /* always procedure */
pub const vpiAssignStmt: i32 = 2; /* quasi-continuous assignment */
pub const vpiAssignment: i32 = 3; /* procedural assignment */
pub const vpiBegin: i32 = 4; /* block statement */
pub const vpiCase: i32 = 5; /* case statement */
pub const vpiCaseItem: i32 = 6; /* case statement item */
pub const vpiConstant: i32 = 7; /* numerical constant or string literal */
pub const vpiContAssign: i32 = 8; /* continuous assignment */
pub const vpiDeassign: i32 = 9; /* deassignment statement */
pub const vpiDefParam: i32 = 10; /* defparam */
pub const vpiDelayControl: i32 = 11; /* delay statement (e.g., #10) */
pub const vpiDisable: i32 = 12; /* named block disable statement */
pub const vpiEventControl: i32 = 13; /* wait on event, e.g., @e */
pub const vpiEventStmt: i32 = 14; /* event trigger, e.g., ->e */
pub const vpiFor: i32 = 15; /* for statement */
pub const vpiForce: i32 = 16; /* force statement */
pub const vpiForever: i32 = 17; /* forever statement */
pub const vpiFork: i32 = 18; /* fork-join block */
pub const vpiFuncCall: i32 = 19; /* function call */
pub const vpiFunction: i32 = 20; /* function */
pub const vpiGate: i32 = 21; /* primitive gate */
pub const vpiIf: i32 = 22; /* if statement */
pub const vpiIfElse: i32 = 23; /* if-else statement */
pub const vpiInitial: i32 = 24; /* initial procedure */
pub const vpiIntegerVar: i32 = 25; /* integer variable */
pub const vpiInterModPath: i32 = 26; /* intermodule wire delay */
pub const vpiIterator: i32 = 27; /* iterator */
pub const vpiIODecl: i32 = 28; /* input/output declaration */
pub const vpiMemory: i32 = 29; /* behavioral memory */
pub const vpiMemoryWord: i32 = 30; /* single word of memory */
pub const vpiModPath: i32 = 31; /* module path for path delays */
pub const vpiModule: i32 = 32; /* module instance */
pub const vpiNamedBegin: i32 = 33; /* named block statement */
pub const vpiNamedEvent: i32 = 34; /* event variable */
pub const vpiNamedFork: i32 = 35; /* named fork-join block */
pub const vpiNet: i32 = 36; /* scalar or vector net */
pub const vpiNetBit: i32 = 37; /* bit of vector net */
pub const vpiNullStmt: i32 = 38; /* a semicolon. Ie. #10 ; */
pub const vpiOperation: i32 = 39; /* behavioral operation */
pub const vpiParamAssign: i32 = 40; /* module parameter assignment */
pub const vpiParameter: i32 = 41; /* module parameter */
pub const vpiPartSelect: i32 = 42; /* part-select */
pub const vpiPathTerm: i32 = 43; /* terminal of module path */
pub const vpiPort: i32 = 44; /* module port */
pub const vpiPortBit: i32 = 45; /* bit of vector module port */
pub const vpiPrimTerm: i32 = 46; /* primitive terminal */
pub const vpiRealVar: i32 = 47; /* real variable */
pub const vpiReg: i32 = 48; /* scalar or vector reg */
pub const vpiRegBit: i32 = 49; /* bit of vector reg */
pub const vpiRelease: i32 = 50; /* release statement */
pub const vpiRepeat: i32 = 51; /* repeat statement */
pub const vpiRepeatControl: i32 = 52; /* repeat control in an assign stmt */
pub const vpiSchedEvent: i32 = 53; /* vpi_put_value() event */
pub const vpiSpecParam: i32 = 54; /* specparam */
pub const vpiSwitch: i32 = 55; /* transistor switch */
pub const vpiSysFuncCall: i32 = 56; /* system function call */
pub const vpiSysTaskCall: i32 = 57; /* system task call */
pub const vpiTableEntry: i32 = 58; /* UDP state table entry */
pub const vpiTask: i32 = 59; /* task */
pub const vpiTaskCall: i32 = 60; /* task call */
pub const vpiTchk: i32 = 61; /* timing check */
pub const vpiTchkTerm: i32 = 62; /* terminal of timing check */
pub const vpiTimeVar: i32 = 63; /* time variable */
pub const vpiTimeQueue: i32 = 64; /* simulation event queue */
pub const vpiUdp: i32 = 65; /* user-defined primitive */
pub const vpiUdpDefn: i32 = 66; /* UDP definition */
pub const vpiUserSystf: i32 = 67; /* user-defined system task/function */
pub const vpiVarSelect: i32 = 68; /* variable array selection */
pub const vpiWait: i32 = 69; /* wait statement */
pub const vpiWhile: i32 = 70; /* while statement */

/********************** object types added with 1364-2001 *********************/

pub const vpiAttribute: i32 = 105; /* attribute of an object */
pub const vpiBitSelect: i32 = 106; /* Bit-select of parameter, var select */
pub const vpiCallback: i32 = 107; /* callback object */
pub const vpiDelayTerm: i32 = 108; /* Delay term which is a load or driver */
pub const vpiDelayDevice: i32 = 109; /* Delay object within a net */
pub const vpiFrame: i32 = 110; /* reentrant task/func frame */
pub const vpiGateArray: i32 = 111; /* gate instance array */
pub const vpiModuleArray: i32 = 112; /* module instance array */
pub const vpiPrimitiveArray: i32 = 113; /* vpiprimitiveArray type */
pub const vpiNetArray: i32 = 114; /* multidimensional net */
pub const vpiRange: i32 = 115; /* range declaration */
pub const vpiRegArray: i32 = 116; /* multidimensional reg */
pub const vpiSwitchArray: i32 = 117; /* switch instance array */
pub const vpiUdpArray: i32 = 118; /* UDP instance array */
pub const vpiContAssignBit: i32 = 128; /* Bit of a vector continuous assignment */
pub const vpiNamedEventArray: i32 = 129; /* multidimensional named event */

/********************** object types added with 1364-2005 *********************/

pub const vpiIndexedPartSelect: i32 = 130; /* Indexed part-select object */
pub const vpiGenScopeArray: i32 = 133; /* array of generated scopes */
pub const vpiGenScope: i32 = 134; /* A generated scope */
pub const vpiGenVar: i32 = 135; /* Object used to instantiate gen scopes */

/*********************************** METHODS **********************************/
/**************** methods used to traverse 1 to 1 relationships ***************/

pub const vpiCondition: i32 = 71; /* condition expression */
pub const vpiDelay: i32 = 72; /* net or gate delay */
pub const vpiElseStmt: i32 = 73; /* else statement */
pub const vpiForIncStmt: i32 = 74; /* increment statement in for loop */
pub const vpiForInitStmt: i32 = 75; /* initialization statement in for loop */
pub const vpiHighConn: i32 = 76; /* higher connection to port */
pub const vpiLhs: i32 = 77; /* left-hand side of assignment */
pub const vpiIndex: i32 = 78; /* index of var select, bit-select, etc. */
pub const vpiLeftRange: i32 = 79; /* left range of vector or part-select */
pub const vpiLowConn: i32 = 80; /* lower connection to port */
pub const vpiParent: i32 = 81; /* parent object */
pub const vpiRhs: i32 = 82; /* right-hand side of assignment */
pub const vpiRightRange: i32 = 83; /* right range of vector or part-select */
pub const vpiScope: i32 = 84; /* containing scope object */
pub const vpiSysTfCall: i32 = 85; /* task function call */
pub const vpiTchkDataTerm: i32 = 86; /* timing check data term */
pub const vpiTchkNotifier: i32 = 87; /* timing check notifier */
pub const vpiTchkRefTerm: i32 = 88; /* timing check reference term */

/************* methods used to traverse 1 to many relationships ***************/

pub const vpiArgument: i32 = 89; /* argument to (system) task/function */
pub const vpiBit: i32 = 90; /* bit of vector net or port */
pub const vpiDriver: i32 = 91; /* driver for a net */
pub const vpiInternalScope: i32 = 92; /* internal scope in module */
pub const vpiLoad: i32 = 93; /* load on net or reg */
pub const vpiModDataPathIn: i32 = 94; /* data terminal of a module path */
pub const vpiModPathIn: i32 = 95; /* Input terminal of a module path */
pub const vpiModPathOut: i32 = 96; /* output terminal of a module path */
pub const vpiOperand: i32 = 97; /* operand of expression */
pub const vpiPortInst: i32 = 98; /* connected port instance */
pub const vpiProcess: i32 = 99; /* process in module, program or interface */
pub const vpiVariables: i32 = 100; /* variables in module */
pub const vpiUse: i32 = 101; /* usage */

/******** methods which can traverse 1 to 1, or 1 to many relationships *******/

pub const vpiExpr: i32 = 102; /* connected expression */
pub const vpiPrimitive: i32 = 103; /* primitive (gate, switch, UDP) */
pub const vpiStmt: i32 = 104; /* statement in process or task */

/************************ methods added with 1364-2001 ************************/

pub const vpiActiveTimeFormat: i32 = 119; /* active $timeformat() system task */
pub const vpiInTerm: i32 = 120; /* To get to a delay device's drivers. */
pub const vpiInstanceArray: i32 = 121; /* vpiInstance arrays */
pub const vpiLocalDriver: i32 = 122; /* local drivers (within a module */
pub const vpiLocalLoad: i32 = 123; /* local loads (within a module */
pub const vpiOutTerm: i32 = 124; /* To get to a delay device's loads. */
pub const vpiPorts: i32 = 125; /* Module port */
pub const vpiSimNet: i32 = 126; /* simulated net after collapsing */
pub const vpiTaskFunc: i32 = 127; /* task/function */

/************************ methods added with 1364-2005 ************************/

pub const vpiBaseExpr: i32 = 131; /* Indexed part-select's base expression */
pub const vpiWidthExpr: i32 = 132; /* Indexed part-select's width expression */

/************************ methods added with 1800-2009 ************************/

pub const vpiAutomatics: i32 = 136; /* Automatic variables of a frame */

/********************************* PROPERTIES *********************************/
/************************** generic object properties *************************/

pub const vpiUndefined: i32 = -1; /* undefined property */
pub const vpiType: i32 = 1; /* type of object */
pub const vpiName: i32 = 2; /* local name of object */
pub const vpiFullName: i32 = 3; /* full hierarchical name */
pub const vpiSize: i32 = 4; /* size of gate, net, port, etc. */
pub const vpiFile: i32 = 5; /* File name in which the object is used*/
pub const vpiLineNo: i32 = 6; /* line number where the object is used */

/***************************** module properties ******************************/

pub const vpiTopModule: i32 = 7; /* top-level module (Boolean) */
pub const vpiCellInstance: i32 = 8; /* cell (Boolean) */
pub const vpiDefName: i32 = 9; /* module definition name */
pub const vpiProtected: i32 = 10; /* source protected module (Boolean) */
pub const vpiTimeUnit: i32 = 11; /* module time unit */
pub const vpiTimePrecision: i32 = 12; /* module time precision */
pub const vpiDefNetType: i32 = 13; /* default net type */
pub const vpiUnconnDrive: i32 = 14; /* unconnected port drive strength */
pub const vpiHighZ: i32 = 1; /* No default drive given */
pub const vpiPull1: i32 = 2; /* default pull1 drive */
pub const vpiPull0: i32 = 3; /* default pull0 drive */
pub const vpiDefFile: i32 = 15; /* File name where the module is defined*/
pub const vpiDefLineNo: i32 = 16; /* line number for module definition */
pub const vpiDefDelayMode: i32 = 47; /* Default delay mode for a module */
pub const vpiDelayModeNone: i32 = 1; /* no delay mode specified */
pub const vpiDelayModePath: i32 = 2; /* path delay mode */
pub const vpiDelayModeDistrib: i32 = 3; /* distributed delay mode */
pub const vpiDelayModeUnit: i32 = 4; /* unit delay mode */
pub const vpiDelayModeZero: i32 = 5; /* zero delay mode */
pub const vpiDelayModeMTM: i32 = 6; /* min:typ:max delay mode */
pub const vpiDefDecayTime: i32 = 48; /* Default decay time for a module */

/*************************** port and net properties **************************/

pub const vpiScalar: i32 = 17; /* scalar (Boolean) */
pub const vpiVector: i32 = 18; /* vector (Boolean) */
pub const vpiExplicitName: i32 = 19; /* port is explicitly named */
pub const vpiDirection: i32 = 20; /* direction of port: */
pub const vpiInput: i32 = 1; /* input */
pub const vpiOutput: i32 = 2; /* output */
pub const vpiInout: i32 = 3; /* inout */
pub const vpiMixedIO: i32 = 4; /* mixed input-output */
pub const vpiNoDirection: i32 = 5; /* no direction */
pub const vpiConnByName: i32 = 21; /* connected by name (Boolean) */

pub const vpiNetType: i32 = 22; /* net subtypes: */
pub const vpiWire: i32 = 1; /* wire net */
pub const vpiWand: i32 = 2; /* wire-and net */
pub const vpiWor: i32 = 3; /* wire-or net */
pub const vpiTri: i32 = 4; /* tri net */
pub const vpiTri0: i32 = 5; /* pull-down net */
pub const vpiTri1: i32 = 6; /* pull-up net */
pub const vpiTriReg: i32 = 7; /* three-state reg net */
pub const vpiTriAnd: i32 = 8; /* three-state wire-and net */
pub const vpiTriOr: i32 = 9; /* three-state wire-or net */
pub const vpiSupply1: i32 = 10; /* supply-1 net */
pub const vpiSupply0: i32 = 11; /* supply-0 net */
pub const vpiNone: i32 = 12; /* no default net type (1364-2001) */
pub const vpiUwire: i32 = 13; /* unresolved wire net (1364-2005) */

pub const vpiExplicitScalared: i32 = 23; /* explicitly scalared (Boolean) */
pub const vpiExplicitVectored: i32 = 24; /* explicitly vectored (Boolean) */
pub const vpiExpanded: i32 = 25; /* expanded vector net (Boolean) */
pub const vpiImplicitDecl: i32 = 26; /* implicitly declared net (Boolean) */
pub const vpiChargeStrength: i32 = 27; /* charge decay strength of net */

/* Defined as part of strengths section.
#define vpiLargeCharge            0x10
#define vpiMediumCharge           0x04
#define vpiSmallCharge            0x02
*/

pub const vpiArray: i32 = 28; /* variable array (Boolean) */
pub const vpiPortIndex: i32 = 29; /* Port index */

/************************ gate and terminal properties ************************/

pub const vpiTermIndex: i32 = 30; /* Index of a primitive terminal */
pub const vpiStrength0: i32 = 31; /* 0-strength of net or gate */
pub const vpiStrength1: i32 = 32; /* 1-strength of net or gate */
pub const vpiPrimType: i32 = 33; /* primitive subtypes: */
pub const vpiAndPrim: i32 = 1; /* and gate */
pub const vpiNandPrim: i32 = 2; /* nand gate */
pub const vpiNorPrim: i32 = 3; /* nor gate */
pub const vpiOrPrim: i32 = 4; /* or gate */
pub const vpiXorPrim: i32 = 5; /* xor gate */
pub const vpiXnorPrim: i32 = 6; /* xnor gate */
pub const vpiBufPrim: i32 = 7; /* buffer */
pub const vpiNotPrim: i32 = 8; /* not gate */
pub const vpiBufif0Prim: i32 = 9; /* zero-enabled buffer */
pub const vpiBufif1Prim: i32 = 10; /* one-enabled buffer */
pub const vpiNotif0Prim: i32 = 11; /* zero-enabled not gate */
pub const vpiNotif1Prim: i32 = 12; /* one-enabled not gate */
pub const vpiNmosPrim: i32 = 13; /* nmos switch */
pub const vpiPmosPrim: i32 = 14; /* pmos switch */
pub const vpiCmosPrim: i32 = 15; /* cmos switch */
pub const vpiRnmosPrim: i32 = 16; /* resistive nmos switch */
pub const vpiRpmosPrim: i32 = 17; /* resistive pmos switch */
pub const vpiRcmosPrim: i32 = 18; /* resistive cmos switch */
pub const vpiRtranPrim: i32 = 19; /* resistive bidirectional */
pub const vpiRtranif0Prim: i32 = 20; /* zero-enable resistive bidirectional */
pub const vpiRtranif1Prim: i32 = 21; /* one-enable resistive bidirectional */
pub const vpiTranPrim: i32 = 22; /* bidirectional */
pub const vpiTranif0Prim: i32 = 23; /* zero-enabled bidirectional */
pub const vpiTranif1Prim: i32 = 24; /* one-enabled bidirectional */
pub const vpiPullupPrim: i32 = 25; /* pullup */
pub const vpiPulldownPrim: i32 = 26; /* pulldown */
pub const vpiSeqPrim: i32 = 27; /* sequential UDP */
pub const vpiCombPrim: i32 = 28; /* combinational UDP */

/**************** path, path terminal, timing check properties ****************/

pub const vpiPolarity: i32 = 34; /* polarity of module path... */
pub const vpiDataPolarity: i32 = 35; /* ...or data path: */
pub const vpiPositive: i32 = 1; /* positive */
pub const vpiNegative: i32 = 2; /* negative */
pub const vpiUnknown: i32 = 3; /* unknown (unspecified) */

pub const vpiEdge: i32 = 36; /* edge type of module path: */
pub const vpiNoEdge: i32 = 0x00; /* no edge */
pub const vpiEdge01: i32 = 0x01; /* 0 -> 1 */
pub const vpiEdge10: i32 = 0x02; /* 1 -> 0 */
pub const vpiEdge0x: i32 = 0x04; /* 0 -> x */
pub const vpiEdgex1: i32 = 0x08; /* x -> 1 */
pub const vpiEdge1x: i32 = 0x10; /* 1 -> x */
pub const vpiEdgex0: i32 = 0x20; /* x -> 0 */
pub const vpiPosedge: i32 = vpiEdgex1 | vpiEdge01 | vpiEdge0x;
pub const vpiNegedge: i32 = vpiEdgex0 | vpiEdge10 | vpiEdge1x;
pub const vpiAnyEdge: i32 = vpiPosedge | vpiNegedge;

pub const vpiPathType: i32 = 37; /* path delay connection subtypes: */
pub const vpiPathFull: i32 = 1; /* ( a *> b ) */
pub const vpiPathParallel: i32 = 2; /* ( a => b ) */

pub const vpiTchkType: i32 = 38; /* timing check subtypes: */
pub const vpiSetup: i32 = 1; /* $setup */
pub const vpiHold: i32 = 2; /* $hold */
pub const vpiPeriod: i32 = 3; /* $period */
pub const vpiWidth: i32 = 4; /* $width */
pub const vpiSkew: i32 = 5; /* $skew */
pub const vpiRecovery: i32 = 6; /* $recovery */
pub const vpiNoChange: i32 = 7; /* $nochange */
pub const vpiSetupHold: i32 = 8; /* $setuphold */
pub const vpiFullskew: i32 = 9; /* $fullskew -- added for 1364-2001 */
pub const vpiRecrem: i32 = 10; /* $recrem   -- added for 1364-2001 */
pub const vpiRemoval: i32 = 11; /* $removal  -- added for 1364-2001 */
pub const vpiTimeskew: i32 = 12; /* $timeskew -- added for 1364-2001 */

/**************************** expression properties ***************************/

pub const vpiOpType: i32 = 39; /* operation subtypes: */
pub const vpiMinusOp: i32 = 1; /* unary minus */
pub const vpiPlusOp: i32 = 2; /* unary plus */
pub const vpiNotOp: i32 = 3; /* unary not */
pub const vpiBitNegOp: i32 = 4; /* bitwise negation */
pub const vpiUnaryAndOp: i32 = 5; /* bitwise reduction AND */
pub const vpiUnaryNandOp: i32 = 6; /* bitwise reduction NAND */
pub const vpiUnaryOrOp: i32 = 7; /* bitwise reduction OR */
pub const vpiUnaryNorOp: i32 = 8; /* bitwise reduction NOR */
pub const vpiUnaryXorOp: i32 = 9; /* bitwise reduction XOR */
pub const vpiUnaryXNorOp: i32 = 10; /* bitwise reduction XNOR */
pub const vpiSubOp: i32 = 11; /* binary subtraction */
pub const vpiDivOp: i32 = 12; /* binary division */
pub const vpiModOp: i32 = 13; /* binary modulus */
pub const vpiEqOp: i32 = 14; /* binary equality */
pub const vpiNeqOp: i32 = 15; /* binary inequality */
pub const vpiCaseEqOp: i32 = 16; /* case (x and z) equality */
pub const vpiCaseNeqOp: i32 = 17; /* case inequality */
pub const vpiGtOp: i32 = 18; /* binary greater than */
pub const vpiGeOp: i32 = 19; /* binary greater than or equal */
pub const vpiLtOp: i32 = 20; /* binary less than */
pub const vpiLeOp: i32 = 21; /* binary less than or equal */
pub const vpiLShiftOp: i32 = 22; /* binary left shift */
pub const vpiRShiftOp: i32 = 23; /* binary right shift */
pub const vpiAddOp: i32 = 24; /* binary addition */
pub const vpiMultOp: i32 = 25; /* binary multiplication */
pub const vpiLogAndOp: i32 = 26; /* binary logical AND */
pub const vpiLogOrOp: i32 = 27; /* binary logical OR */
pub const vpiBitAndOp: i32 = 28; /* binary bitwise AND */
pub const vpiBitOrOp: i32 = 29; /* binary bitwise OR */
pub const vpiBitXorOp: i32 = 30; /* binary bitwise XOR */
pub const vpiBitXNorOp: i32 = 31; /* binary bitwise XNOR */
pub const vpiBitXnorOp: i32 = vpiBitXNorOp; /* added with 1364-2001 */
pub const vpiConditionOp: i32 = 32; /* ternary conditional */
pub const vpiConcatOp: i32 = 33; /* n-ary concatenation */
pub const vpiMultiConcatOp: i32 = 34; /* repeated concatenation */
pub const vpiEventOrOp: i32 = 35; /* event OR */
pub const vpiNullOp: i32 = 36; /* null operation */
pub const vpiListOp: i32 = 37; /* list of expressions */
pub const vpiMinTypMaxOp: i32 = 38; /* min:typ:max: delay expression */
pub const vpiPosedgeOp: i32 = 39; /* posedge */
pub const vpiNegedgeOp: i32 = 40; /* negedge */
pub const vpiArithLShiftOp: i32 = 41; /* arithmetic left shift  (1364-2001) */
pub const vpiArithRShiftOp: i32 = 42; /* arithmetic right shift (1364-2001) */
pub const vpiPowerOp: i32 = 43; /* arithmetic power op    (1364-2001) */

pub const vpiConstType: i32 = 40; /* constant subtypes: */
pub const vpiDecConst: i32 = 1; /* decimal integer */
pub const vpiRealConst: i32 = 2; /* real */
pub const vpiBinaryConst: i32 = 3; /* binary integer */
pub const vpiOctConst: i32 = 4; /* octal integer */
pub const vpiHexConst: i32 = 5; /* hexadecimal integer */
pub const vpiStringConst: i32 = 6; /* string literal */
pub const vpiIntConst: i32 = 7; /* integer constant (1364-2001) */
pub const vpiTimeConst: i32 = 8; /* time constant */
pub const vpiBlocking: i32 = 41; /* blocking assignment (Boolean) */
pub const vpiCaseType: i32 = 42; /* case statement subtypes: */
pub const vpiCaseExact: i32 = 1; /* exact match */
pub const vpiCaseX: i32 = 2; /* ignore X's */
pub const vpiCaseZ: i32 = 3; /* ignore Z's */
pub const vpiNetDeclAssign: i32 = 43; /* assign part of decl (Boolean) */

/************************** task/function properties **************************/

pub const vpiFuncType: i32 = 44; /* function & system function type */
pub const vpiIntFunc: i32 = 1; /* returns integer */
pub const vpiRealFunc: i32 = 2; /* returns real */
pub const vpiTimeFunc: i32 = 3; /* returns time */
pub const vpiSizedFunc: i32 = 4; /* returns an arbitrary size */
pub const vpiSizedSignedFunc: i32 = 5; /* returns sized signed value */

/** alias 1364-1995 system function subtypes to 1364-2001 function subtypes ***/

pub const vpiSysFuncType: i32 = vpiFuncType;
pub const vpiSysFuncInt: i32 = vpiIntFunc;
pub const vpiSysFuncReal: i32 = vpiRealFunc;
pub const vpiSysFuncTime: i32 = vpiTimeFunc;
pub const vpiSysFuncSized: i32 = vpiSizedFunc;

pub const vpiUserDefn: i32 = 45; /*user-defined system task/func(Boolean)*/
pub const vpiScheduled: i32 = 46; /* object still scheduled (Boolean) */

/*********************** properties added with 1364-2001 **********************/

pub const vpiActive: i32 = 49; /* reentrant task/func frame is active */
pub const vpiAutomatic: i32 = 50; /* task/func obj is automatic */
pub const vpiCell: i32 = 51; /* configuration cell */
pub const vpiConfig: i32 = 52; /* configuration config file */
pub const vpiConstantSelect: i32 = 53; /* (Boolean) bit-select or part-select
                                       indices are constant expressions */
pub const vpiDecompile: i32 = 54; /* decompile the object */
pub const vpiDefAttribute: i32 = 55; /* Attribute defined for the obj */
pub const vpiDelayType: i32 = 56; /* delay subtype */
pub const vpiModPathDelay: i32 = 1; /* module path delay */
pub const vpiInterModPathDelay: i32 = 2; /* intermodule path delay */
pub const vpiMIPDelay: i32 = 3; /* module input port delay */
pub const vpiIteratorType: i32 = 57; /* object type of an iterator */
pub const vpiLibrary: i32 = 58; /* configuration library */
pub const vpiOffset: i32 = 60; /* offset from LSB */
pub const vpiResolvedNetType: i32 = 61; /* net subtype after resolution, returns
                                        same subtypes as vpiNetType */
pub const vpiSaveRestartID: i32 = 62; /* unique ID for save/restart data */
pub const vpiSaveRestartLocation: i32 = 63; /* name of save/restart data file */
/* vpiValid,vpiValidTrue,vpiValidFalse were deprecated in 1800-2009 */
pub const vpiValid: i32 = 64; /* reentrant task/func frame or automatic
                              variable is valid */
pub const vpiValidFalse: i32 = 0;
pub const vpiValidTrue: i32 = 1;
pub const vpiSigned: i32 = 65; /* TRUE for vpiIODecl and any object in
                               the expression class if the object
                               has the signed attribute */
pub const vpiLocalParam: i32 = 70; /* TRUE when a param is declared as a
                                   localparam */
pub const vpiModPathHasIfNone: i32 = 71; /* Mod path has an ifnone statement */

/*********************** properties added with 1364-2005 **********************/

pub const vpiIndexedPartSelectType: i32 = 72; /* Indexed part-select type */
pub const vpiPosIndexed: i32 = 1; /* +: */
pub const vpiNegIndexed: i32 = 2; /* -: */
pub const vpiIsMemory: i32 = 73; /* TRUE for a one-dimensional reg array */
pub const vpiIsProtected: i32 = 74; /* TRUE for protected design information */

/*************** vpi_control() constants (added with 1364-2001) ***************/

pub const vpiStop: i32 = 66; /* execute simulator's $stop */
pub const vpiFinish: i32 = 67; /* execute simulator's $finish */
pub const vpiReset: i32 = 68; /* execute simulator's $reset */
pub const vpiSetInteractiveScope: i32 = 69; /* set simulator's interactive scope */

/**************************** I/O related defines *****************************/

pub const VPI_MCD_STDOUT: u32 = 0x00000001;

/*************************** STRUCTURE DEFINITIONS ****************************/

/******************************* time structure *******************************/

#[repr(C)]
pub struct s_vpi_time {
    pub r#type: PLI_INT32, /* [vpiScaledRealTime, vpiSimTime,
                           vpiSuppressTime] */
    pub high: PLI_UINT32,
    pub low: PLI_UINT32, /* for vpiSimTime */
    pub real: c_double,  /* for vpiScaledRealTime */
}
pub type p_vpi_time = *mut s_vpi_time;

/* time types */
pub const vpiScaledRealTime: i32 = 1;
pub const vpiSimTime: i32 = 2;
pub const vpiSuppressTime: i32 = 3;

/***************************** value structures *******************************/

/* vector value */
#[repr(C)]
pub struct s_vpi_vecval {
    /* following fields are repeated enough times to contain vector */
    pub aval: PLI_UINT32,
    pub bval: PLI_UINT32, /* bit encoding: ab: 00=0, 10=1, 11=X, 01=Z */
}
pub type p_vpi_vecval = *mut s_vpi_vecval;

/* strength (scalar) value */
#[repr(C)]
pub struct s_vpi_strengthval {
    pub logic: PLI_INT32, /* vpi[0,1,X,Z] */
    pub s0: PLI_INT32,
    pub s1: PLI_INT32, /* refer to strength coding below */
}
pub type p_vpi_strengthval = *mut s_vpi_strengthval;

/* strength values */
pub const vpiSupplyDrive: i32 = 0x80;
pub const vpiStrongDrive: i32 = 0x40;
pub const vpiPullDrive: i32 = 0x20;
pub const vpiWeakDrive: i32 = 0x08;
pub const vpiLargeCharge: i32 = 0x10;
pub const vpiMediumCharge: i32 = 0x04;
pub const vpiSmallCharge: i32 = 0x02;
pub const vpiHiZ: i32 = 0x01;

/* generic value */

#[repr(C)]
pub union s_vpi_value_value {
    pub str: *const PLI_BYTE8,       /* string value */
    pub scalar: PLI_INT32,           /* vpi[0,1,X,Z] */
    pub integer: PLI_INT32,          /* integer value */
    pub real: c_double,              /* real value */
    pub time: p_vpi_time,            /* time value */
    pub vector: p_vpi_vecval,        /* vector value */
    pub strength: p_vpi_strengthval, /* strength value */
    pub misc: *const PLI_BYTE8,      /* ...other */
}

#[repr(C)]
pub struct s_vpi_value {
    pub format: PLI_INT32, /* vpi[[Bin,Oct,Dec,Hex]Str,Scalar,Int,Real,String,
                           Vector,Strength,Suppress,Time,ObjType]Val */
    pub value: s_vpi_value_value,
}
pub type p_vpi_value = *mut s_vpi_value;

#[repr(C)]
pub union s_vpi_arrayvalue_value {
    pub integers: *mut PLI_INT32,  /* integer values */
    pub shortints: *mut PLI_INT16, /* short integer values */
    pub longints: *mut PLI_INT64,  /* long integer values */
    pub rawvals: *mut PLI_BYTE8,   /* 2/4-state vector elements */
    pub vectors: p_vpi_vecval,     /* 4-state vector elements */
    pub times: p_vpi_time,         /* time values */
    pub reals: *mut c_double,      /* real values */
    pub shortreals: *mut c_float,  /* short real values */
}

#[repr(C)]
pub struct s_vpi_arrayvalue {
    pub format: PLI_UINT32, /* vpi[Int,Real,Time,ShortInt,LongInt,ShortReal,
                            RawTwoState,RawFourState]Val */
    pub flags: PLI_UINT32, /* array bit flags- vpiUserAllocFlag */
    pub value: s_vpi_arrayvalue_value,
}
pub type p_vpi_arrayvalue = *mut s_vpi_arrayvalue;

/* value formats */

pub const vpiBinStrVal: u32 = 1;
pub const vpiOctStrVal: u32 = 2;
pub const vpiDecStrVal: u32 = 3;
pub const vpiHexStrVal: u32 = 4;
pub const vpiScalarVal: u32 = 5;
pub const vpiIntVal: u32 = 6;
pub const vpiRealVal: u32 = 7;
pub const vpiStringVal: u32 = 8;
pub const vpiVectorVal: u32 = 9;
pub const vpiStrengthVal: u32 = 10;
pub const vpiTimeVal: u32 = 11;
pub const vpiObjTypeVal: u32 = 12;
pub const vpiSuppressVal: u32 = 13;
pub const vpiShortIntVal: u32 = 14;
pub const vpiLongIntVal: u32 = 15;
pub const vpiShortRealVal: u32 = 16;
pub const vpiRawTwoStateVal: u32 = 17;
pub const vpiRawFourStateVal: u32 = 18;

/* delay modes */
pub const vpiNoDelay: i32 = 1;
pub const vpiInertialDelay: i32 = 2;
pub const vpiTransportDelay: i32 = 3;
pub const vpiPureTransportDelay: i32 = 4;

/* force and release flags */
pub const vpiForceFlag: i32 = 5;
pub const vpiReleaseFlag: i32 = 6;

/* scheduled event cancel flag */

pub const vpiCancelEvent: i32 = 7;

/* bit mask for the flags argument to vpi_put_value() */

pub const vpiReturnEvent: u16 = 0x1000;

/* bit flags for vpi_get_value_array flags field */

pub const vpiUserAllocFlag: u16 = 0x2000;

/* bit flags for vpi_put_value_array flags field */

pub const vpiOneValue: u16 = 0x4000;
pub const vpiPropagateOff: u16 = 0x8000;

/* scalar values */

pub const vpi0: i32 = 0;
pub const vpi1: i32 = 1;
pub const vpiZ: i32 = 2;
pub const vpiX: i32 = 3;
pub const vpiH: i32 = 4;
pub const vpiL: i32 = 5;
pub const vpiDontCare: i32 = 6;
/*
#define vpiNoChange           7   Defined under vpiTchkType, but
                                  can be used here.
*/

/*********************** system task/function structure ***********************/

#[repr(C)]
pub struct s_vpi_systf_data {
    pub r#type: PLI_INT32, /* vpiSysTask, vpiSysFunc */
    pub sysfunctype: PLI_INT32, /* vpiSysTask, vpi[Int,Real,Time,Sized,
                           SizedSigned]Func */
    pub tfname: *const PLI_BYTE8, /* first character must be '$' */
    pub calltf: Option<extern "C" fn(*const PLI_BYTE8) -> PLI_INT32>,
    pub compiletf: Option<extern "C" fn(*const PLI_BYTE8) -> PLI_INT32>,
    pub sizetf: Option<extern "C" fn(*const PLI_BYTE8) -> PLI_INT32>, /* for sized function callbacks only */
    pub user_data: *const PLI_BYTE8,
}
pub type p_vpi_systf_data = *const s_vpi_systf_data;

pub const vpiSysTask: i32 = 1;
pub const vpiSysFunc: i32 = 2;

/* the subtypes are defined under the vpiFuncType property */

/**************** SystemVerilog execution information structure ***************/

#[repr(C)]
pub struct s_vpi_vlog_info {
    pub argc: i32,
    pub argv: *const *const PLI_BYTE8,
    pub product: *const PLI_BYTE8,
    pub version: *const PLI_BYTE8,
}
pub type p_vpi_vlog_info = *const s_vpi_vlog_info;

/*********************** PLI error information structure **********************/

#[repr(C)]
pub struct s_vpi_error_info {
    pub state: PLI_INT32, /* vpi[Compile,PLI,Run] */
    pub level: PLI_INT32, /* vpi[Notice,Warning,Error,System,Internal] */
    pub message: *const PLI_BYTE8,
    pub product: *const PLI_BYTE8,
    pub code: *const PLI_BYTE8,
    pub file: *const PLI_BYTE8,
    pub line: PLI_INT32,
}
pub type p_vpi_error_info = *const s_vpi_error_info;

/* state when error occurred */

pub const vpiCompile: i32 = 1;
pub const vpiPLI: i32 = 2;
pub const vpiRun: i32 = 3;

/* error severity levels */

pub const vpiNotice: i32 = 1;
pub const vpiWarning: i32 = 2;
pub const vpiError: i32 = 3;
pub const vpiSystem: i32 = 4;
pub const vpiInternal: i32 = 5;

/**************************** callback structures *****************************/

/* normal callback structure */

#[repr(C)]
pub struct s_cb_data {
    pub reason: PLI_INT32, /* callback reason */
    pub cb_rtn: Option<unsafe extern "C" fn(p_cb_data) -> PLI_INT32>, /* call routine */
    pub obj: vpiHandle,    /* trigger object */
    pub time: p_vpi_time,  /* callback time */
    pub value: p_vpi_value, /* trigger object value */
    pub index: PLI_INT32,  /* index of the memory word or
                           var select that changed */
    pub user_data: *const PLI_BYTE8,
}
pub type p_cb_data = *const s_cb_data;

/****************************** CALLBACK REASONS ******************************/

/***************************** Simulation related *****************************/

pub const cbValueChange: c_int = 1;
pub const cbStmt: c_int = 2;
pub const cbForce: c_int = 3;
pub const cbRelease: c_int = 4;

/******************************** Time related ********************************/

pub const cbAtStartOfSimTime: c_int = 5;
pub const cbReadWriteSynch: c_int = 6;
pub const cbReadOnlySynch: c_int = 7;
pub const cbNextSimTime: c_int = 8;
pub const cbAfterDelay: c_int = 9;

/******************************* Action related *******************************/

pub const cbEndOfCompile: c_int = 10;
pub const cbStartOfSimulation: c_int = 11;
pub const cbEndOfSimulation: c_int = 12;
pub const cbError: c_int = 13;
pub const cbTchkViolation: c_int = 14;
pub const cbStartOfSave: c_int = 15;
pub const cbEndOfSave: c_int = 16;
pub const cbStartOfRestart: c_int = 17;
pub const cbEndOfRestart: c_int = 18;
pub const cbStartOfReset: c_int = 19;
pub const cbEndOfReset: c_int = 20;
pub const cbEnterInteractive: c_int = 21;
pub const cbExitInteractive: c_int = 22;
pub const cbInteractiveScopeChange: c_int = 23;
pub const cbUnresolvedSystf: c_int = 24;

/**************************** Added with 1364-2001 ****************************/

pub const cbAssign: c_int = 25;
pub const cbDeassign: c_int = 26;
pub const cbDisable: c_int = 27;
pub const cbPLIError: c_int = 28;
pub const cbSignal: c_int = 29;

/**************************** Added with 1364-2005 ****************************/
pub const cbNBASynch: c_int = 30;
pub const cbAtEndOfSimTime: c_int = 31;

/**************************** FUNCTION DECLARATIONS ***************************/

/* Include compatibility mode macro definitions. */
//#include "vpi_compatibility.h"

/* callback related */
extern "C" {
    pub fn vpi_register_cb(cb_data_p: p_cb_data) -> vpiHandle;
    pub fn vpi_remove_cb(cb_obj: vpiHandle) -> PLI_INT32;

    /* for obtaining handles */
    pub fn vpi_handle_by_name(name: *const PLI_BYTE8, scope: vpiHandle) -> vpiHandle;
    pub fn vpi_handle_by_index(object: vpiHandle, indx: PLI_INT32) -> vpiHandle;

    /* for traversing relationships */

    pub fn vpi_handle(r#type: PLI_INT32, refHandle: vpiHandle) -> vpiHandle;
    pub fn vpi_iterate(r#type: PLI_INT32, refHandle: vpiHandle) -> vpiHandle;
    pub fn vpi_scan(iterator: vpiHandle) -> vpiHandle;

    /* for processing properties */

    pub fn vpi_get(property: PLI_INT32, object: vpiHandle) -> PLI_INT32;
    pub fn vpi_get_str(property: PLI_INT32, object: vpiHandle) -> *const PLI_UBYTE8;

    /* value processing */

    pub fn vpi_get_value(expr: vpiHandle, value_p: p_vpi_value);
    pub fn vpi_put_value(
        object: vpiHandle,
        value_p: p_vpi_value,
        time_p: p_vpi_time,
        flags: PLI_INT32,
    ) -> vpiHandle;

    /* time processing */

    pub fn vpi_get_time(object: vpiHandle, time_p: p_vpi_time);

    /* I/O routines */

    pub fn vpi_mcd_open(fileName: *const PLI_BYTE8) -> PLI_UINT32;
    pub fn vpi_mcd_close(mcd: PLI_UINT32) -> PLI_UINT32;
    pub fn vpi_mcd_printf(mcd: PLI_UINT32, format: *const PLI_BYTE8, ...) -> PLI_INT32;
    pub fn vpi_printf(format: *const PLI_BYTE8, ...) -> PLI_INT32;

    /* utility routines */

    pub fn vpi_chk_error(error_info_p: p_vpi_error_info) -> PLI_INT32;
    /* vpi_free_object() was deprecated in 1800-2009 */
    pub fn vpi_free_object(object: vpiHandle) -> PLI_INT32;
    pub fn vpi_release_handle(object: vpiHandle) -> PLI_INT32;
    pub fn vpi_get_vlog_info(vlog_info_p: p_vpi_vlog_info) -> PLI_INT32;

    /* routines added with 1364-2001 */

    // TODO: Needs c_variadic https://github.com/rust-lang/rust/issues/44930
    //pub fn vpi_vprintf(format: *const PLI_BYTE8, ap: va_list) -> PLI_INT32;
    // TODO: Needs c_variadic https://github.com/rust-lang/rust/issues/44930
    //pub fn vpi_mcd_vprintf(mcd: PLI_UINT32, format: *const PLI_BYTE8, ap: va_list)
    //    -> PLI_INT32;
    pub fn vpi_flush() -> PLI_INT32;
    pub fn vpi_mcd_flush(mcd: PLI_UINT32) -> PLI_INT32;
    pub fn vpi_control(operation: PLI_INT32, ...) -> PLI_INT32;

    /****************************** GLOBAL VARIABLES ******************************/

    /* array of function pointers, last pointer should be null */
    pub static mut vlog_startup_routines: [Option<unsafe extern "C" fn()>; 1usize];
}
