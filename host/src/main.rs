// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use methods::{
    ZKHIRE_ELF, ZKHIRE_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};

struct ApplicantData {
    name: String,
    ssn: u32,
    birth_date: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Get applicant information.
    let applicant = ApplicantData {
        name: "Joe".to_string(),
        ssn: 123456789,
        birth_date: "01/01/1970".to_string()
    }

    // Call Vendor API using applicant information to retrieve the input to the program.
    let checker_receipt = checker.get_input(applicant);

    // Verify vendor's receipt.
    checker_receipt.verify(CHECKR_ID)?;

    // Add to database
    zkdb.add_receipt(checker_receipt);

    // TODO: We need to wait for more vendors to respond, add the receipts to the zkDB

    // Aggregate vendor's receipt with other receipts.
    // - Adding the receipt metadata to Executor environment.
    let env = ExecutorEnv::builder()
    .add_receipt(checker_receipt)?
    .write(&input)
    .write(&checker_receipt.journal.decode()).build()?;

    let prover = default_prover();

    // Run the aggregator
    let hire_receipt = prover.prove_elf(env, ZKHIRE_ELF)?;

    let delete_receipt = zkdb.delete(checker_receipt);

    // Aggregate vendor's receipt with other receipts.
    let env = ExecutorEnv::builder()
    .add_receipt(hire_receipt)?
    .add_receipt(delete_receipt)?
    .write(&hire_receipt).build()?;

    // Produce a receipt by proving the specified ELF binary.
    let passport_receipt = prover.prove_elf(env, ZKHIRE_MAKE_PASSPORT_ELF)?;

    // Make user happy
    zkhire.send_passport(passport_receipt)?;
}
