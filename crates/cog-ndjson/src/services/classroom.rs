use serde::{Deserialize, Serialize};

/// Classroom service protocol requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum ClassroomRequest {
    CoursesList {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        role: Option<String>,
    },
    CoursesGet {
        course_id: String,
    },
    CoursesCreate {
        name: String,
    },
    CoursesUpdate {
        course_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    CoursesArchive {
        course_id: String,
    },
    CoursesUnarchive {
        course_id: String,
    },
    #[cfg(feature = "destructive-permanent")]
    CoursesDelete {
        course_id: String,
    },
    CoursesUrl {
        course_id: String,
    },
    Roster {
        course_id: String,
        #[serde(default)]
        students_only: bool,
    },
    StudentsAdd {
        course_id: String,
        user_id: String,
    },
    TeachersAdd {
        course_id: String,
        user_id: String,
    },
    CourseworkList {
        course_id: String,
    },
    CourseworkGet {
        course_id: String,
        coursework_id: String,
    },
    CourseworkCreate {
        course_id: String,
        title: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        work_type: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        state: Option<String>,
    },
    CourseworkUpdate {
        course_id: String,
        coursework_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },
    SubmissionsList {
        course_id: String,
        coursework_id: String,
    },
    SubmissionsGet {
        course_id: String,
        coursework_id: String,
        submission_id: String,
    },
    SubmissionsGrade {
        course_id: String,
        coursework_id: String,
        submission_id: String,
        grade: String,
    },
    SubmissionsReturn {
        course_id: String,
        coursework_id: String,
        submission_id: String,
    },
    SubmissionsTurnIn {
        course_id: String,
        coursework_id: String,
        submission_id: String,
    },
    SubmissionsReclaim {
        course_id: String,
        coursework_id: String,
        submission_id: String,
    },
    AnnouncementsList {
        course_id: String,
    },
    AnnouncementsCreate {
        course_id: String,
        text: String,
    },
    AnnouncementsUpdate {
        course_id: String,
        announcement_id: String,
        text: String,
    },
    TopicsList {
        course_id: String,
    },
    TopicsCreate {
        course_id: String,
        name: String,
    },
    TopicsUpdate {
        course_id: String,
        topic_id: String,
        name: String,
    },
    InvitationsList,
    InvitationsCreate {
        course_id: String,
        user_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        role: Option<String>,
    },
    InvitationsAccept {
        invitation_id: String,
    },
    GuardiansList {
        student_id: String,
    },
    GuardiansGet {
        student_id: String,
        guardian_id: String,
    },
    GuardiansDelete {
        student_id: String,
        guardian_id: String,
    },
    GuardianInvitationsList {
        student_id: String,
    },
    GuardianInvitationsCreate {
        student_id: String,
        email: String,
    },
    ProfileGet {
        user_id: String,
    },
}
