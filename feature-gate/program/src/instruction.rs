//! Program instructions

use {
    num_enum::{IntoPrimitive, TryFromPrimitive},
    solana_program::{
        instruction::{AccountMeta, Instruction},
        program_error::ProgramError,
        pubkey::Pubkey,
        system_program,
    },
};

/// Feature Gate program instructions
#[derive(Clone, Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum FeatureGateInstruction {
    /// Queue a feature for activation by allocating and assigning a feature
    /// account.
    ///
    /// Note: This instruction expects the account to be owned by the system
    /// program.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[w+s]`    Feature account (must be a system account)
    ///   1. `[w+s]`    Payer (for rent lamports)
    ///   2. `[]`       System program
    ActivateFeature,
    /// Revoke a pending feature activation.
    ///
    /// A "pending" feature activation is a feature account that has been
    /// allocated and assigned, but hasn't yet been updated by the runtime
    /// with an `activation_slot`.
    ///
    /// Features that _have_ been activated by the runtime cannot be revoked.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[w+s]`    Feature account
    ///   1. `[w]`      Destination (for rent lamports)
    RevokePendingActivation,
}
impl FeatureGateInstruction {
    /// Unpacks a byte buffer into a
    /// [FeatureGateInstruction](enum.FeatureGateInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        if input.len() != 1 {
            return Err(ProgramError::InvalidInstructionData);
        }
        Self::try_from(input[0]).map_err(|_| ProgramError::InvalidInstructionData)
    }

    /// Packs a [FeatureGateInstruction](enum.FeatureGateInstruction.html) into
    /// a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        vec![self.to_owned().into()]
    }
}

/// Creates an 'ActivateFeature' instruction.
pub fn activate_feature(feature_id: &Pubkey, payer: &Pubkey) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*feature_id, true),
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    let data = FeatureGateInstruction::ActivateFeature.pack();

    Instruction {
        program_id: crate::id(),
        accounts,
        data,
    }
}

/// Creates a 'RevokePendingActivation' instruction.
pub fn revoke_pending_activation(feature_id: &Pubkey, destination: &Pubkey) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*feature_id, true),
        AccountMeta::new(*destination, false),
    ];

    let data = FeatureGateInstruction::RevokePendingActivation.pack();

    Instruction {
        program_id: crate::id(),
        accounts,
        data,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_pack_unpack(instruction: &FeatureGateInstruction) {
        let packed = instruction.pack();
        let unpacked = FeatureGateInstruction::unpack(&packed).unwrap();
        assert_eq!(instruction, &unpacked);
    }

    #[test]
    fn test_pack_unpack_activate_feature() {
        test_pack_unpack(&FeatureGateInstruction::ActivateFeature);
    }

    #[test]
    fn test_pack_unpack_revoke_pending_activation() {
        test_pack_unpack(&FeatureGateInstruction::RevokePendingActivation);
    }
}
