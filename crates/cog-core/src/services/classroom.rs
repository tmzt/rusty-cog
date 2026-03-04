//! Google Classroom API service client.
//!
//! Wraps the Classroom REST API v1.
//! <https://developers.google.com/classroom/reference/rest>

use crate::error::{Error, Result};
use crate::http::HttpClient;
use serde::Deserialize;

const BASE: &str = "https://classroom.googleapis.com/v1";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListCoursesResponse {
    #[serde(default)]
    courses: Vec<serde_json::Value>,
    #[serde(default)]
    next_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListStudentsResponse {
    #[serde(default)]
    students: Vec<serde_json::Value>,
    #[serde(default)]
    next_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListTeachersResponse {
    #[serde(default)]
    teachers: Vec<serde_json::Value>,
    #[serde(default)]
    next_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListCourseworkResponse {
    #[serde(default)]
    course_work: Vec<serde_json::Value>,
    #[serde(default)]
    next_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListSubmissionsResponse {
    #[serde(default)]
    student_submissions: Vec<serde_json::Value>,
    #[serde(default)]
    next_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListAnnouncementsResponse {
    #[serde(default)]
    announcements: Vec<serde_json::Value>,
    #[serde(default)]
    next_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListTopicsResponse {
    #[serde(default)]
    topic: Vec<serde_json::Value>,
    #[serde(default)]
    next_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListInvitationsResponse {
    #[serde(default)]
    invitations: Vec<serde_json::Value>,
    #[serde(default)]
    next_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListGuardiansResponse {
    #[serde(default)]
    guardians: Vec<serde_json::Value>,
    #[serde(default)]
    next_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListGuardianInvitationsResponse {
    #[serde(default)]
    guardian_invitations: Vec<serde_json::Value>,
    #[serde(default)]
    next_page_token: Option<String>,
}

/// Async client for the Google Classroom API.
#[derive(Debug, Clone)]
pub struct ClassroomService {
    http: HttpClient,
    token: String,
}

impl ClassroomService {
    /// Create a new `ClassroomService`.
    pub fn new(http: HttpClient, token: String) -> Self {
        Self { http, token }
    }

    fn parse<T: serde::de::DeserializeOwned>(&self, bytes: &[u8]) -> Result<T> {
        serde_json::from_slice(bytes).map_err(|e| Error::Other(format!("JSON parse error: {e}")))
    }

    // -- courses ------------------------------------------------------------

    /// List courses.
    pub async fn courses_list(
        &self,
        page_size: Option<u32>,
        page_token: Option<&str>,
        course_states: Option<&[String]>,
    ) -> Result<(Vec<serde_json::Value>, Option<String>)> {
        let mut url = format!("{BASE}/courses");
        let mut sep = '?';
        if let Some(n) = page_size {
            url.push_str(&format!("{sep}pageSize={n}"));
            sep = '&';
        }
        if let Some(pt) = page_token {
            url.push_str(&format!("{sep}pageToken={}", urlencoding(pt)));
            sep = '&';
        }
        if let Some(states) = course_states {
            for state in states {
                url.push_str(&format!("{sep}courseStates={}", urlencoding(state)));
                sep = '&';
            }
        }
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListCoursesResponse = self.parse(&resp)?;
        Ok((list.courses, list.next_page_token))
    }

    /// Get a single course by ID.
    pub async fn courses_get(&self, course_id: &str) -> Result<serde_json::Value> {
        let url = format!("{BASE}/courses/{course_id}");
        let resp = self.http.get(&url, &self.token).await?;
        self.parse(&resp)
    }

    /// Create a new course.
    pub async fn courses_create(&self, course: &serde_json::Value) -> Result<serde_json::Value> {
        let body = serde_json::to_vec(course).map_err(|e| Error::Other(e.to_string()))?;
        let url = format!("{BASE}/courses");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Update a course.
    pub async fn courses_update(
        &self,
        course_id: &str,
        course: &serde_json::Value,
        update_mask: &str,
    ) -> Result<serde_json::Value> {
        let body = serde_json::to_vec(course).map_err(|e| Error::Other(e.to_string()))?;
        let url = format!(
            "{BASE}/courses/{course_id}?updateMask={}",
            urlencoding(update_mask)
        );
        let resp = self.http.patch(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Archive a course.
    pub async fn courses_archive(&self, course_id: &str) -> Result<serde_json::Value> {
        let body = serde_json::to_vec(&serde_json::json!({
            "courseState": "ARCHIVED"
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = format!("{BASE}/courses/{course_id}?updateMask=courseState");
        let resp = self.http.patch(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Unarchive (reactivate) a course.
    pub async fn courses_unarchive(&self, course_id: &str) -> Result<serde_json::Value> {
        let body = serde_json::to_vec(&serde_json::json!({
            "courseState": "ACTIVE"
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = format!("{BASE}/courses/{course_id}?updateMask=courseState");
        let resp = self.http.patch(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Permanently delete a course.
    #[cfg(feature = "destructive-permanent")]
    pub async fn courses_delete(&self, course_id: &str) -> Result<()> {
        let url = format!("{BASE}/courses/{course_id}");
        self.http.delete(&url, &self.token).await?;
        Ok(())
    }

    // -- roster -------------------------------------------------------------

    /// Get the roster (students + teachers) for a course.
    pub async fn roster(&self, course_id: &str) -> Result<serde_json::Value> {
        let (students, _) = self.students_list(course_id, None, None).await?;
        let (teachers, _) = self.teachers_list(course_id, None, None).await?;
        Ok(serde_json::json!({
            "students": students,
            "teachers": teachers,
        }))
    }

    /// List students in a course.
    pub async fn students_list(
        &self,
        course_id: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<(Vec<serde_json::Value>, Option<String>)> {
        let mut url = format!("{BASE}/courses/{course_id}/students");
        let mut sep = '?';
        if let Some(n) = page_size {
            url.push_str(&format!("{sep}pageSize={n}"));
            sep = '&';
        }
        if let Some(pt) = page_token {
            url.push_str(&format!("{sep}pageToken={}", urlencoding(pt)));
        }
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListStudentsResponse = self.parse(&resp)?;
        Ok((list.students, list.next_page_token))
    }

    /// List teachers in a course.
    pub async fn teachers_list(
        &self,
        course_id: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<(Vec<serde_json::Value>, Option<String>)> {
        let mut url = format!("{BASE}/courses/{course_id}/teachers");
        let mut sep = '?';
        if let Some(n) = page_size {
            url.push_str(&format!("{sep}pageSize={n}"));
            sep = '&';
        }
        if let Some(pt) = page_token {
            url.push_str(&format!("{sep}pageToken={}", urlencoding(pt)));
        }
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListTeachersResponse = self.parse(&resp)?;
        Ok((list.teachers, list.next_page_token))
    }

    /// Add a student to a course.
    pub async fn students_add(
        &self,
        course_id: &str,
        user_id: &str,
    ) -> Result<serde_json::Value> {
        let body = serde_json::to_vec(&serde_json::json!({
            "userId": user_id,
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = format!("{BASE}/courses/{course_id}/students");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Add a teacher to a course.
    pub async fn teachers_add(
        &self,
        course_id: &str,
        user_id: &str,
    ) -> Result<serde_json::Value> {
        let body = serde_json::to_vec(&serde_json::json!({
            "userId": user_id,
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = format!("{BASE}/courses/{course_id}/teachers");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    // -- coursework ---------------------------------------------------------

    /// List coursework in a course.
    pub async fn coursework_list(
        &self,
        course_id: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<(Vec<serde_json::Value>, Option<String>)> {
        let mut url = format!("{BASE}/courses/{course_id}/courseWork");
        let mut sep = '?';
        if let Some(n) = page_size {
            url.push_str(&format!("{sep}pageSize={n}"));
            sep = '&';
        }
        if let Some(pt) = page_token {
            url.push_str(&format!("{sep}pageToken={}", urlencoding(pt)));
        }
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListCourseworkResponse = self.parse(&resp)?;
        Ok((list.course_work, list.next_page_token))
    }

    /// Get a single coursework item.
    pub async fn coursework_get(
        &self,
        course_id: &str,
        coursework_id: &str,
    ) -> Result<serde_json::Value> {
        let url = format!("{BASE}/courses/{course_id}/courseWork/{coursework_id}");
        let resp = self.http.get(&url, &self.token).await?;
        self.parse(&resp)
    }

    /// Create a coursework item.
    pub async fn coursework_create(
        &self,
        course_id: &str,
        coursework: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let body = serde_json::to_vec(coursework).map_err(|e| Error::Other(e.to_string()))?;
        let url = format!("{BASE}/courses/{course_id}/courseWork");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Update a coursework item.
    pub async fn coursework_update(
        &self,
        course_id: &str,
        coursework_id: &str,
        coursework: &serde_json::Value,
        update_mask: &str,
    ) -> Result<serde_json::Value> {
        let body = serde_json::to_vec(coursework).map_err(|e| Error::Other(e.to_string()))?;
        let url = format!(
            "{BASE}/courses/{course_id}/courseWork/{coursework_id}?updateMask={}",
            urlencoding(update_mask)
        );
        let resp = self.http.patch(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    // -- submissions --------------------------------------------------------

    /// List student submissions for a coursework item.
    pub async fn submissions_list(
        &self,
        course_id: &str,
        coursework_id: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<(Vec<serde_json::Value>, Option<String>)> {
        let mut url = format!(
            "{BASE}/courses/{course_id}/courseWork/{coursework_id}/studentSubmissions"
        );
        let mut sep = '?';
        if let Some(n) = page_size {
            url.push_str(&format!("{sep}pageSize={n}"));
            sep = '&';
        }
        if let Some(pt) = page_token {
            url.push_str(&format!("{sep}pageToken={}", urlencoding(pt)));
        }
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListSubmissionsResponse = self.parse(&resp)?;
        Ok((list.student_submissions, list.next_page_token))
    }

    /// Get a single student submission.
    pub async fn submissions_get(
        &self,
        course_id: &str,
        coursework_id: &str,
        submission_id: &str,
    ) -> Result<serde_json::Value> {
        let url = format!(
            "{BASE}/courses/{course_id}/courseWork/{coursework_id}/studentSubmissions/{submission_id}"
        );
        let resp = self.http.get(&url, &self.token).await?;
        self.parse(&resp)
    }

    /// Grade a student submission.
    pub async fn submissions_grade(
        &self,
        course_id: &str,
        coursework_id: &str,
        submission_id: &str,
        assigned_grade: Option<f64>,
        draft_grade: Option<f64>,
    ) -> Result<serde_json::Value> {
        let mut obj = serde_json::json!({});
        let mut mask_parts = Vec::new();
        if let Some(g) = assigned_grade {
            obj["assignedGrade"] = serde_json::json!(g);
            mask_parts.push("assignedGrade");
        }
        if let Some(g) = draft_grade {
            obj["draftGrade"] = serde_json::json!(g);
            mask_parts.push("draftGrade");
        }
        let body = serde_json::to_vec(&obj).map_err(|e| Error::Other(e.to_string()))?;
        let url = format!(
            "{BASE}/courses/{course_id}/courseWork/{coursework_id}/studentSubmissions/{submission_id}?updateMask={}",
            mask_parts.join(",")
        );
        let resp = self.http.patch(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Return a student submission to the student.
    pub async fn submissions_return(
        &self,
        course_id: &str,
        coursework_id: &str,
        submission_id: &str,
    ) -> Result<()> {
        let url = format!(
            "{BASE}/courses/{course_id}/courseWork/{coursework_id}/studentSubmissions/{submission_id}:return"
        );
        self.http.post(&url, &self.token, &[]).await?;
        Ok(())
    }

    /// Turn in a student submission.
    pub async fn submissions_turn_in(
        &self,
        course_id: &str,
        coursework_id: &str,
        submission_id: &str,
    ) -> Result<()> {
        let url = format!(
            "{BASE}/courses/{course_id}/courseWork/{coursework_id}/studentSubmissions/{submission_id}:turnIn"
        );
        self.http.post(&url, &self.token, &[]).await?;
        Ok(())
    }

    /// Reclaim a turned-in student submission.
    pub async fn submissions_reclaim(
        &self,
        course_id: &str,
        coursework_id: &str,
        submission_id: &str,
    ) -> Result<()> {
        let url = format!(
            "{BASE}/courses/{course_id}/courseWork/{coursework_id}/studentSubmissions/{submission_id}:reclaim"
        );
        self.http.post(&url, &self.token, &[]).await?;
        Ok(())
    }

    // -- announcements ------------------------------------------------------

    /// List announcements in a course.
    pub async fn announcements_list(
        &self,
        course_id: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<(Vec<serde_json::Value>, Option<String>)> {
        let mut url = format!("{BASE}/courses/{course_id}/announcements");
        let mut sep = '?';
        if let Some(n) = page_size {
            url.push_str(&format!("{sep}pageSize={n}"));
            sep = '&';
        }
        if let Some(pt) = page_token {
            url.push_str(&format!("{sep}pageToken={}", urlencoding(pt)));
        }
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListAnnouncementsResponse = self.parse(&resp)?;
        Ok((list.announcements, list.next_page_token))
    }

    /// Create an announcement.
    pub async fn announcements_create(
        &self,
        course_id: &str,
        announcement: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let body =
            serde_json::to_vec(announcement).map_err(|e| Error::Other(e.to_string()))?;
        let url = format!("{BASE}/courses/{course_id}/announcements");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Update an announcement.
    pub async fn announcements_update(
        &self,
        course_id: &str,
        announcement_id: &str,
        announcement: &serde_json::Value,
        update_mask: &str,
    ) -> Result<serde_json::Value> {
        let body =
            serde_json::to_vec(announcement).map_err(|e| Error::Other(e.to_string()))?;
        let url = format!(
            "{BASE}/courses/{course_id}/announcements/{announcement_id}?updateMask={}",
            urlencoding(update_mask)
        );
        let resp = self.http.patch(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    // -- topics -------------------------------------------------------------

    /// List topics in a course.
    pub async fn topics_list(
        &self,
        course_id: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<(Vec<serde_json::Value>, Option<String>)> {
        let mut url = format!("{BASE}/courses/{course_id}/topics");
        let mut sep = '?';
        if let Some(n) = page_size {
            url.push_str(&format!("{sep}pageSize={n}"));
            sep = '&';
        }
        if let Some(pt) = page_token {
            url.push_str(&format!("{sep}pageToken={}", urlencoding(pt)));
        }
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListTopicsResponse = self.parse(&resp)?;
        Ok((list.topic, list.next_page_token))
    }

    /// Create a topic.
    pub async fn topics_create(
        &self,
        course_id: &str,
        name: &str,
    ) -> Result<serde_json::Value> {
        let body = serde_json::to_vec(&serde_json::json!({ "name": name }))
            .map_err(|e| Error::Other(e.to_string()))?;
        let url = format!("{BASE}/courses/{course_id}/topics");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Update a topic.
    pub async fn topics_update(
        &self,
        course_id: &str,
        topic_id: &str,
        name: &str,
    ) -> Result<serde_json::Value> {
        let body = serde_json::to_vec(&serde_json::json!({ "name": name }))
            .map_err(|e| Error::Other(e.to_string()))?;
        let url = format!(
            "{BASE}/courses/{course_id}/topics/{topic_id}?updateMask=name"
        );
        let resp = self.http.patch(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    // -- invitations --------------------------------------------------------

    /// List invitations.
    pub async fn invitations_list(
        &self,
        course_id: Option<&str>,
        user_id: Option<&str>,
        page_token: Option<&str>,
    ) -> Result<(Vec<serde_json::Value>, Option<String>)> {
        let mut url = format!("{BASE}/invitations");
        let mut sep = '?';
        if let Some(cid) = course_id {
            url.push_str(&format!("{sep}courseId={}", urlencoding(cid)));
            sep = '&';
        }
        if let Some(uid) = user_id {
            url.push_str(&format!("{sep}userId={}", urlencoding(uid)));
            sep = '&';
        }
        if let Some(pt) = page_token {
            url.push_str(&format!("{sep}pageToken={}", urlencoding(pt)));
        }
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListInvitationsResponse = self.parse(&resp)?;
        Ok((list.invitations, list.next_page_token))
    }

    /// Create an invitation.
    pub async fn invitations_create(
        &self,
        course_id: &str,
        user_id: &str,
        role: &str,
    ) -> Result<serde_json::Value> {
        let body = serde_json::to_vec(&serde_json::json!({
            "courseId": course_id,
            "userId": user_id,
            "role": role,
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = format!("{BASE}/invitations");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Accept an invitation.
    pub async fn invitations_accept(&self, invitation_id: &str) -> Result<()> {
        let url = format!("{BASE}/invitations/{invitation_id}:accept");
        self.http.post(&url, &self.token, &[]).await?;
        Ok(())
    }

    // -- guardians ----------------------------------------------------------

    /// List guardians for a student.
    pub async fn guardians_list(
        &self,
        student_id: &str,
        page_token: Option<&str>,
    ) -> Result<(Vec<serde_json::Value>, Option<String>)> {
        let mut url = format!("{BASE}/userProfiles/{student_id}/guardians");
        if let Some(pt) = page_token {
            url.push_str(&format!("?pageToken={}", urlencoding(pt)));
        }
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListGuardiansResponse = self.parse(&resp)?;
        Ok((list.guardians, list.next_page_token))
    }

    /// Get a single guardian.
    pub async fn guardians_get(
        &self,
        student_id: &str,
        guardian_id: &str,
    ) -> Result<serde_json::Value> {
        let url = format!("{BASE}/userProfiles/{student_id}/guardians/{guardian_id}");
        let resp = self.http.get(&url, &self.token).await?;
        self.parse(&resp)
    }

    /// Delete a guardian.
    pub async fn guardians_delete(
        &self,
        student_id: &str,
        guardian_id: &str,
    ) -> Result<()> {
        let url = format!("{BASE}/userProfiles/{student_id}/guardians/{guardian_id}");
        self.http.delete(&url, &self.token).await?;
        Ok(())
    }

    // -- guardian invitations -----------------------------------------------

    /// List guardian invitations for a student.
    pub async fn guardian_invitations_list(
        &self,
        student_id: &str,
        states: Option<&[String]>,
        page_token: Option<&str>,
    ) -> Result<(Vec<serde_json::Value>, Option<String>)> {
        let mut url = format!("{BASE}/userProfiles/{student_id}/guardianInvitations");
        let mut sep = '?';
        if let Some(ss) = states {
            for s in ss {
                url.push_str(&format!("{sep}states={}", urlencoding(s)));
                sep = '&';
            }
        }
        if let Some(pt) = page_token {
            url.push_str(&format!("{sep}pageToken={}", urlencoding(pt)));
        }
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListGuardianInvitationsResponse = self.parse(&resp)?;
        Ok((list.guardian_invitations, list.next_page_token))
    }

    /// Create a guardian invitation.
    pub async fn guardian_invitations_create(
        &self,
        student_id: &str,
        invited_email_address: &str,
    ) -> Result<serde_json::Value> {
        let body = serde_json::to_vec(&serde_json::json!({
            "invitedEmailAddress": invited_email_address,
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = format!("{BASE}/userProfiles/{student_id}/guardianInvitations");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    // -- user profiles ------------------------------------------------------

    /// Get a user profile.
    pub async fn profile_get(&self, user_id: &str) -> Result<serde_json::Value> {
        let url = format!("{BASE}/userProfiles/{user_id}");
        let resp = self.http.get(&url, &self.token).await?;
        self.parse(&resp)
    }
}

fn urlencoding(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}
