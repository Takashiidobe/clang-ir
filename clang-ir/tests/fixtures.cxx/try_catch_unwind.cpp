// A try block with only a typed catch (no catch-all) gets an implicit
// #cir.unwind handler region for the uncaught case.
int risky();

int f() {
  try {
    return risky();
  } catch (int e) {
    return e;
  }
}
