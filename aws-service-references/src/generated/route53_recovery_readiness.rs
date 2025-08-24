// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum Route53RecoveryReadinessActions {
    CreateCell,
    CreateCrossAccountAuthorization,
    CreateReadinessCheck,
    CreateRecoveryGroup,
    CreateResourceSet,
    DeleteCell,
    DeleteCrossAccountAuthorization,
    DeleteReadinessCheck,
    DeleteRecoveryGroup,
    DeleteResourceSet,
    GetArchitectureRecommendations,
    GetCell,
    GetCellReadinessSummary,
    GetReadinessCheck,
    GetReadinessCheckResourceStatus,
    GetReadinessCheckStatus,
    GetRecoveryGroup,
    GetRecoveryGroupReadinessSummary,
    GetResourceSet,
    ListCells,
    ListCrossAccountAuthorizations,
    ListReadinessChecks,
    ListRecoveryGroups,
    ListResourceSets,
    ListRules,
    ListTagsForResources,
    TagResource,
    UntagResource,
    UpdateCell,
    UpdateReadinessCheck,
    UpdateRecoveryGroup,
    UpdateResourceSet,
}
impl std::fmt::Display for Route53RecoveryReadinessActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Route53RecoveryReadinessActions::CreateCell => {
                write!(f, "route53-recovery-readiness:CreateCell")
            }
            Route53RecoveryReadinessActions::CreateCrossAccountAuthorization => write!(
                f,
                "route53-recovery-readiness:CreateCrossAccountAuthorization"
            ),
            Route53RecoveryReadinessActions::CreateReadinessCheck => {
                write!(f, "route53-recovery-readiness:CreateReadinessCheck")
            }
            Route53RecoveryReadinessActions::CreateRecoveryGroup => {
                write!(f, "route53-recovery-readiness:CreateRecoveryGroup")
            }
            Route53RecoveryReadinessActions::CreateResourceSet => {
                write!(f, "route53-recovery-readiness:CreateResourceSet")
            }
            Route53RecoveryReadinessActions::DeleteCell => {
                write!(f, "route53-recovery-readiness:DeleteCell")
            }
            Route53RecoveryReadinessActions::DeleteCrossAccountAuthorization => write!(
                f,
                "route53-recovery-readiness:DeleteCrossAccountAuthorization"
            ),
            Route53RecoveryReadinessActions::DeleteReadinessCheck => {
                write!(f, "route53-recovery-readiness:DeleteReadinessCheck")
            }
            Route53RecoveryReadinessActions::DeleteRecoveryGroup => {
                write!(f, "route53-recovery-readiness:DeleteRecoveryGroup")
            }
            Route53RecoveryReadinessActions::DeleteResourceSet => {
                write!(f, "route53-recovery-readiness:DeleteResourceSet")
            }
            Route53RecoveryReadinessActions::GetArchitectureRecommendations => write!(
                f,
                "route53-recovery-readiness:GetArchitectureRecommendations"
            ),
            Route53RecoveryReadinessActions::GetCell => {
                write!(f, "route53-recovery-readiness:GetCell")
            }
            Route53RecoveryReadinessActions::GetCellReadinessSummary => {
                write!(f, "route53-recovery-readiness:GetCellReadinessSummary")
            }
            Route53RecoveryReadinessActions::GetReadinessCheck => {
                write!(f, "route53-recovery-readiness:GetReadinessCheck")
            }
            Route53RecoveryReadinessActions::GetReadinessCheckResourceStatus => write!(
                f,
                "route53-recovery-readiness:GetReadinessCheckResourceStatus"
            ),
            Route53RecoveryReadinessActions::GetReadinessCheckStatus => {
                write!(f, "route53-recovery-readiness:GetReadinessCheckStatus")
            }
            Route53RecoveryReadinessActions::GetRecoveryGroup => {
                write!(f, "route53-recovery-readiness:GetRecoveryGroup")
            }
            Route53RecoveryReadinessActions::GetRecoveryGroupReadinessSummary => write!(
                f,
                "route53-recovery-readiness:GetRecoveryGroupReadinessSummary"
            ),
            Route53RecoveryReadinessActions::GetResourceSet => {
                write!(f, "route53-recovery-readiness:GetResourceSet")
            }
            Route53RecoveryReadinessActions::ListCells => {
                write!(f, "route53-recovery-readiness:ListCells")
            }
            Route53RecoveryReadinessActions::ListCrossAccountAuthorizations => write!(
                f,
                "route53-recovery-readiness:ListCrossAccountAuthorizations"
            ),
            Route53RecoveryReadinessActions::ListReadinessChecks => {
                write!(f, "route53-recovery-readiness:ListReadinessChecks")
            }
            Route53RecoveryReadinessActions::ListRecoveryGroups => {
                write!(f, "route53-recovery-readiness:ListRecoveryGroups")
            }
            Route53RecoveryReadinessActions::ListResourceSets => {
                write!(f, "route53-recovery-readiness:ListResourceSets")
            }
            Route53RecoveryReadinessActions::ListRules => {
                write!(f, "route53-recovery-readiness:ListRules")
            }
            Route53RecoveryReadinessActions::ListTagsForResources => {
                write!(f, "route53-recovery-readiness:ListTagsForResources")
            }
            Route53RecoveryReadinessActions::TagResource => {
                write!(f, "route53-recovery-readiness:TagResource")
            }
            Route53RecoveryReadinessActions::UntagResource => {
                write!(f, "route53-recovery-readiness:UntagResource")
            }
            Route53RecoveryReadinessActions::UpdateCell => {
                write!(f, "route53-recovery-readiness:UpdateCell")
            }
            Route53RecoveryReadinessActions::UpdateReadinessCheck => {
                write!(f, "route53-recovery-readiness:UpdateReadinessCheck")
            }
            Route53RecoveryReadinessActions::UpdateRecoveryGroup => {
                write!(f, "route53-recovery-readiness:UpdateRecoveryGroup")
            }
            Route53RecoveryReadinessActions::UpdateResourceSet => {
                write!(f, "route53-recovery-readiness:UpdateResourceSet")
            }
        }
    }
}
