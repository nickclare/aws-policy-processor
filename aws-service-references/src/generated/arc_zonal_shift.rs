// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ArcZonalShiftActions {
    CancelPracticeRun,
    CancelZonalShift,
    CreatePracticeRunConfiguration,
    DeletePracticeRunConfiguration,
    GetAutoshiftObserverNotificationStatus,
    GetManagedResource,
    ListAutoshifts,
    ListManagedResources,
    ListZonalShifts,
    StartPracticeRun,
    StartZonalShift,
    UpdateAutoshiftObserverNotificationStatus,
    UpdatePracticeRunConfiguration,
    UpdateZonalAutoshiftConfiguration,
    UpdateZonalShift,
}
impl std::fmt::Display for ArcZonalShiftActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArcZonalShiftActions::CancelPracticeRun => {
                write!(f, "arc-zonal-shift:CancelPracticeRun")
            }
            ArcZonalShiftActions::CancelZonalShift => write!(f, "arc-zonal-shift:CancelZonalShift"),
            ArcZonalShiftActions::CreatePracticeRunConfiguration => {
                write!(f, "arc-zonal-shift:CreatePracticeRunConfiguration")
            }
            ArcZonalShiftActions::DeletePracticeRunConfiguration => {
                write!(f, "arc-zonal-shift:DeletePracticeRunConfiguration")
            }
            ArcZonalShiftActions::GetAutoshiftObserverNotificationStatus => {
                write!(f, "arc-zonal-shift:GetAutoshiftObserverNotificationStatus")
            }
            ArcZonalShiftActions::GetManagedResource => {
                write!(f, "arc-zonal-shift:GetManagedResource")
            }
            ArcZonalShiftActions::ListAutoshifts => write!(f, "arc-zonal-shift:ListAutoshifts"),
            ArcZonalShiftActions::ListManagedResources => {
                write!(f, "arc-zonal-shift:ListManagedResources")
            }
            ArcZonalShiftActions::ListZonalShifts => write!(f, "arc-zonal-shift:ListZonalShifts"),
            ArcZonalShiftActions::StartPracticeRun => write!(f, "arc-zonal-shift:StartPracticeRun"),
            ArcZonalShiftActions::StartZonalShift => write!(f, "arc-zonal-shift:StartZonalShift"),
            ArcZonalShiftActions::UpdateAutoshiftObserverNotificationStatus => write!(
                f,
                "arc-zonal-shift:UpdateAutoshiftObserverNotificationStatus"
            ),
            ArcZonalShiftActions::UpdatePracticeRunConfiguration => {
                write!(f, "arc-zonal-shift:UpdatePracticeRunConfiguration")
            }
            ArcZonalShiftActions::UpdateZonalAutoshiftConfiguration => {
                write!(f, "arc-zonal-shift:UpdateZonalAutoshiftConfiguration")
            }
            ArcZonalShiftActions::UpdateZonalShift => write!(f, "arc-zonal-shift:UpdateZonalShift"),
        }
    }
}
