// -*- mode: C++; c-file-style: "cc-mode" -*-
/// \file
/// \brief Verilated Shim: Expose C++ static interface as C functions.

#include <verilated.h>

// METHODS - User called
extern "C" {

    /// Select initial value of otherwise uninitialized signals.
    ////
    /// 0 = Set to zeros
    /// 1 = Set all bits to one
    /// 2 = Randomize all bits
void
verilated_set_rand_reset(int val) {
  Verilated::randReset(val);
}

/// Return randReset value
int
verilated_rand_reset() {
  return Verilated::randReset();
}

/// Enable debug of internal verilated code
void
verilated_set_debug(int level) {
  return Verilated::debug(level);
}

/// Return debug value
int
verilated_debug() {
  return Verilated::debug();
}

/// Enable calculation of unused signals
void
verilated_set_calc_unused_sigs(int flag) {
  Verilated::calcUnusedSigs(flag != 0);
}

/// Return calcUnusedSigs value
int
verilated_calc_unused_sigs() {
  return Verilated::calcUnusedSigs() ? 1 : 0;
}

/// Did the simulation $finish?
void
verilated_set_got_finish(int flag) {
  Verilated::gotFinish(flag != 0);
}

/// Return if got a $finish
int
verilated_got_finish() {
  return Verilated::gotFinish() ? 1 : 0;
}

/// Allow traces to at some point be enabled (disables some optimizations)
void
verilated_trace_ever_on(int flag) {
  Verilated::traceEverOn(flag != 0);
}

/// Enable/disable assertions
void
verilated_set_assert_on(int flag) {
  Verilated::assertOn(flag != 0);
}

int
verilated_assert_on() {
  return Verilated::assertOn() ? 1 : 0;
}

/// Enable/disable vpi fatal
void
verilated_set_fatal_on_vpi_error(int flag) {
  Verilated::fatalOnVpiError(flag != 0);
}

int
verilated_fatal_on_vpi_error() {
  return Verilated::fatalOnVpiError() ? 1 : 0;
}

#if (VERILATOR_VERSION_MAJOR >= 5) || (VERILATOR_VERSION_MAJOR == 4 && VERILATOR_VERSION_MINOR >= 38)
typedef void (*voidp_cb)(void*);  // Callback type for below

/// Callbacks to run on global flush
void
verilated_add_flush_cb(voidp_cb cb, void* datap) {
  Verilated::addFlushCb(cb, datap);
}

void
verilated_remove_flush_cb(voidp_cb cb, void* datap) {
  Verilated::removeFlushCb(cb, datap);
}

void
verilator_run_flush_callbacks() {
  Verilated::runFlushCallbacks();
}

/// Callbacks to run prior to termination
void
verilated_add_exit_cb(voidp_cb cb, void* datap) {
  Verilated::addExitCb(cb, datap);
}

void
verilated_remove_exit_cb(voidp_cb cb, void* datap) {
  Verilated::removeExitCb(cb, datap);
}

void
verilator_run_exit_callbacks() {
  Verilated::runExitCallbacks();
}
#else // !((VERILATOR_VERSION_MAJOR >= 5) || (VERILATOR_VERSION_MAJOR == 4 && VERILATOR_VERSION_MINOR >= 38))
/// Flush callback for VCD waves
void
verilated_flush_cb(VerilatedVoidCb cb) {
  Verilated::flushCb(cb);
}

void
verilated_flush_call() {
  Verilated::flushCall();
}
#endif // VERILATOR_VERSION_MAJOR == 4 && VERILATOR_VERSION_MINOR >= 38

/// Record command line arguments, for retrieval by $test$plusargs/$value$plusargs
void
verilated_command_args(int argc, const char** argv) {
  Verilated::commandArgs(argc, argv);
}

//    static CommandArgValues* getCommandArgs() {return &s_args;}

/// Match plusargs with a given prefix. Returns static char* valid only for a single call
const char*
verilated_command_args_plus_match(const char* prefixp) {
  return Verilated::commandArgsPlusMatch(prefixp);
}

/// Produce name & version for (at least) VPI
const char*
verilated_product_name() {
  return Verilated::productName();
}

const char*
verilated_product_version() {
  return Verilated::productVersion();
}

/// For debugging, print much of the Verilator internal state.
/// The output of this function may change in future
/// releases - contact the authors before production use.
void
verilated_internals_dump() {
  Verilated::internalsDump();
}

/// For debugging, print text list of all scope names with
/// dpiImport/Export context.  This function may change in future
/// releases - contact the authors before production use.
void
verilated_scopes_dump() {
  Verilated::scopesDump();
}

}


