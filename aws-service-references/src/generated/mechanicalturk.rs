// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum MechanicalturkActions {
    AcceptQualificationRequest,
    ApproveAssignment,
    AssociateQualificationWithWorker,
    CreateAdditionalAssignmentsForHit,
    CreateHit,
    CreateHitType,
    CreateHitWithHitType,
    CreateQualificationType,
    CreateWorkerBlock,
    DeleteHit,
    DeleteQualificationType,
    DeleteWorkerBlock,
    DisassociateQualificationFromWorker,
    GetAccountBalance,
    GetAssignment,
    GetFileUploadUrl,
    GetHit,
    GetQualificationScore,
    GetQualificationType,
    ListAssignmentsForHit,
    ListBonusPayments,
    ListHiTs,
    ListHiTsForQualificationType,
    ListQualificationRequests,
    ListQualificationTypes,
    ListReviewPolicyResultsForHit,
    ListReviewableHiTs,
    ListWorkerBlocks,
    ListWorkersWithQualificationType,
    NotifyWorkers,
    RejectAssignment,
    RejectQualificationRequest,
    SendBonus,
    SendTestEventNotification,
    UpdateExpirationForHit,
    UpdateHitReviewStatus,
    UpdateHitTypeOfHit,
    UpdateNotificationSettings,
    UpdateQualificationType,
}
impl std::fmt::Display for MechanicalturkActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MechanicalturkActions::AcceptQualificationRequest => {
                write!(f, "mechanicalturk:AcceptQualificationRequest")
            }
            MechanicalturkActions::ApproveAssignment => {
                write!(f, "mechanicalturk:ApproveAssignment")
            }
            MechanicalturkActions::AssociateQualificationWithWorker => {
                write!(f, "mechanicalturk:AssociateQualificationWithWorker")
            }
            MechanicalturkActions::CreateAdditionalAssignmentsForHit => {
                write!(f, "mechanicalturk:CreateAdditionalAssignmentsForHIT")
            }
            MechanicalturkActions::CreateHit => write!(f, "mechanicalturk:CreateHIT"),
            MechanicalturkActions::CreateHitType => write!(f, "mechanicalturk:CreateHITType"),
            MechanicalturkActions::CreateHitWithHitType => {
                write!(f, "mechanicalturk:CreateHITWithHITType")
            }
            MechanicalturkActions::CreateQualificationType => {
                write!(f, "mechanicalturk:CreateQualificationType")
            }
            MechanicalturkActions::CreateWorkerBlock => {
                write!(f, "mechanicalturk:CreateWorkerBlock")
            }
            MechanicalturkActions::DeleteHit => write!(f, "mechanicalturk:DeleteHIT"),
            MechanicalturkActions::DeleteQualificationType => {
                write!(f, "mechanicalturk:DeleteQualificationType")
            }
            MechanicalturkActions::DeleteWorkerBlock => {
                write!(f, "mechanicalturk:DeleteWorkerBlock")
            }
            MechanicalturkActions::DisassociateQualificationFromWorker => {
                write!(f, "mechanicalturk:DisassociateQualificationFromWorker")
            }
            MechanicalturkActions::GetAccountBalance => {
                write!(f, "mechanicalturk:GetAccountBalance")
            }
            MechanicalturkActions::GetAssignment => write!(f, "mechanicalturk:GetAssignment"),
            MechanicalturkActions::GetFileUploadUrl => write!(f, "mechanicalturk:GetFileUploadURL"),
            MechanicalturkActions::GetHit => write!(f, "mechanicalturk:GetHIT"),
            MechanicalturkActions::GetQualificationScore => {
                write!(f, "mechanicalturk:GetQualificationScore")
            }
            MechanicalturkActions::GetQualificationType => {
                write!(f, "mechanicalturk:GetQualificationType")
            }
            MechanicalturkActions::ListAssignmentsForHit => {
                write!(f, "mechanicalturk:ListAssignmentsForHIT")
            }
            MechanicalturkActions::ListBonusPayments => {
                write!(f, "mechanicalturk:ListBonusPayments")
            }
            MechanicalturkActions::ListHiTs => write!(f, "mechanicalturk:ListHITs"),
            MechanicalturkActions::ListHiTsForQualificationType => {
                write!(f, "mechanicalturk:ListHITsForQualificationType")
            }
            MechanicalturkActions::ListQualificationRequests => {
                write!(f, "mechanicalturk:ListQualificationRequests")
            }
            MechanicalturkActions::ListQualificationTypes => {
                write!(f, "mechanicalturk:ListQualificationTypes")
            }
            MechanicalturkActions::ListReviewPolicyResultsForHit => {
                write!(f, "mechanicalturk:ListReviewPolicyResultsForHIT")
            }
            MechanicalturkActions::ListReviewableHiTs => {
                write!(f, "mechanicalturk:ListReviewableHITs")
            }
            MechanicalturkActions::ListWorkerBlocks => write!(f, "mechanicalturk:ListWorkerBlocks"),
            MechanicalturkActions::ListWorkersWithQualificationType => {
                write!(f, "mechanicalturk:ListWorkersWithQualificationType")
            }
            MechanicalturkActions::NotifyWorkers => write!(f, "mechanicalturk:NotifyWorkers"),
            MechanicalturkActions::RejectAssignment => write!(f, "mechanicalturk:RejectAssignment"),
            MechanicalturkActions::RejectQualificationRequest => {
                write!(f, "mechanicalturk:RejectQualificationRequest")
            }
            MechanicalturkActions::SendBonus => write!(f, "mechanicalturk:SendBonus"),
            MechanicalturkActions::SendTestEventNotification => {
                write!(f, "mechanicalturk:SendTestEventNotification")
            }
            MechanicalturkActions::UpdateExpirationForHit => {
                write!(f, "mechanicalturk:UpdateExpirationForHIT")
            }
            MechanicalturkActions::UpdateHitReviewStatus => {
                write!(f, "mechanicalturk:UpdateHITReviewStatus")
            }
            MechanicalturkActions::UpdateHitTypeOfHit => {
                write!(f, "mechanicalturk:UpdateHITTypeOfHIT")
            }
            MechanicalturkActions::UpdateNotificationSettings => {
                write!(f, "mechanicalturk:UpdateNotificationSettings")
            }
            MechanicalturkActions::UpdateQualificationType => {
                write!(f, "mechanicalturk:UpdateQualificationType")
            }
        }
    }
}
