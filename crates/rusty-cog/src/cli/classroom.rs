use clap::{Args, Subcommand};

/// Google Classroom operations.
#[derive(Args, Debug)]
pub struct ClassroomArgs {
    #[command(subcommand)]
    pub command: ClassroomCommands,
}

#[derive(Subcommand, Debug)]
pub enum ClassroomCommands {
    /// Course management
    Courses {
        #[command(subcommand)]
        sub: CoursesCommands,
    },

    /// List course roster (students + teachers)
    Roster {
        /// Course ID
        #[arg(long)]
        course_id: String,
    },

    /// Student management
    Students {
        #[command(subcommand)]
        sub: StudentsCommands,
    },

    /// Teacher management
    Teachers {
        #[command(subcommand)]
        sub: TeachersCommands,
    },

    /// Coursework (assignments) management
    Coursework {
        #[command(subcommand)]
        sub: CourseworkCommands,
    },

    /// Course materials management
    Materials {
        #[command(subcommand)]
        sub: MaterialsCommands,
    },

    /// Student submissions management
    Submissions {
        #[command(subcommand)]
        sub: SubmissionsCommands,
    },

    /// Course announcements management
    Announcements {
        #[command(subcommand)]
        sub: AnnouncementsCommands,
    },

    /// Course topic management
    Topics {
        #[command(subcommand)]
        sub: TopicsCommands,
    },

    /// Invitation management
    Invitations {
        #[command(subcommand)]
        sub: InvitationsCommands,
    },

    /// Guardian management
    Guardians {
        #[command(subcommand)]
        sub: GuardiansCommands,
    },

    /// Guardian invitation management
    GuardianInvitations {
        #[command(subcommand)]
        sub: GuardianInvitationsCommands,
    },

    /// User profile lookup
    Profile {
        #[command(subcommand)]
        sub: ProfileCommands,
    },
}

/// Subcommands under `classroom courses`.
#[derive(Subcommand, Debug)]
pub enum CoursesCommands {
    /// List courses
    List {
        /// Filter by course state (ACTIVE, ARCHIVED, PROVISIONED, DECLINED, SUSPENDED)
        #[arg(long)]
        state: Option<String>,

        /// Filter by student email
        #[arg(long)]
        student: Option<String>,

        /// Filter by teacher email
        #[arg(long)]
        teacher: Option<String>,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Get a course by ID
    Get {
        /// Course ID
        id: String,
    },

    /// Create a new course
    Create {
        /// Course name
        #[arg(long)]
        name: String,

        /// Section name
        #[arg(long)]
        section: Option<String>,

        /// Course description
        #[arg(long)]
        description: Option<String>,

        /// Room/location
        #[arg(long)]
        room: Option<String>,

        /// Owner email (defaults to current user)
        #[arg(long)]
        owner: Option<String>,
    },

    /// Update a course
    Update {
        /// Course ID
        id: String,

        /// New course name
        #[arg(long)]
        name: Option<String>,

        /// New section name
        #[arg(long)]
        section: Option<String>,

        /// New description
        #[arg(long)]
        description: Option<String>,

        /// New room/location
        #[arg(long)]
        room: Option<String>,

        /// New course state
        #[arg(long)]
        state: Option<String>,
    },

    /// Delete a course
    #[cfg(feature = "destructive-permanent")]
    Delete {
        /// Course ID
        id: String,
    },
}

/// Subcommands under `classroom students`.
#[derive(Subcommand, Debug)]
pub enum StudentsCommands {
    /// List students in a course
    List {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Get a student
    Get {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Student user ID or email
        user_id: String,
    },

    /// Add a student to a course
    Add {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Student email
        #[arg(long)]
        email: String,
    },

    /// Remove a student from a course
    Remove {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Student user ID or email
        user_id: String,
    },
}

/// Subcommands under `classroom teachers`.
#[derive(Subcommand, Debug)]
pub enum TeachersCommands {
    /// List teachers in a course
    List {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Get a teacher
    Get {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Teacher user ID or email
        user_id: String,
    },

    /// Add a teacher to a course
    Add {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Teacher email
        #[arg(long)]
        email: String,
    },

    /// Remove a teacher from a course
    Remove {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Teacher user ID or email
        user_id: String,
    },
}

/// Subcommands under `classroom coursework`.
#[derive(Subcommand, Debug)]
pub enum CourseworkCommands {
    /// List coursework in a course
    List {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,

        /// Filter by state (PUBLISHED, DRAFT, DELETED)
        #[arg(long)]
        state: Option<String>,
    },

    /// Get a coursework item
    Get {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Coursework ID
        id: String,
    },

    /// Create a coursework item (assignment)
    Create {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Title
        #[arg(long)]
        title: String,

        /// Description
        #[arg(long)]
        description: Option<String>,

        /// Work type (ASSIGNMENT, SHORT_ANSWER_QUESTION, MULTIPLE_CHOICE_QUESTION)
        #[arg(long)]
        work_type: Option<String>,

        /// Maximum points
        #[arg(long)]
        max_points: Option<f64>,

        /// Due date (YYYY-MM-DD)
        #[arg(long)]
        due_date: Option<String>,

        /// Due time (HH:MM)
        #[arg(long)]
        due_time: Option<String>,

        /// Topic ID
        #[arg(long)]
        topic_id: Option<String>,

        /// State (PUBLISHED, DRAFT)
        #[arg(long)]
        state: Option<String>,
    },

    /// Update a coursework item
    Update {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Coursework ID
        id: String,

        /// New title
        #[arg(long)]
        title: Option<String>,

        /// New description
        #[arg(long)]
        description: Option<String>,

        /// New state
        #[arg(long)]
        state: Option<String>,

        /// New due date
        #[arg(long)]
        due_date: Option<String>,
    },

    /// Delete a coursework item
    Delete {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Coursework ID
        id: String,
    },
}

/// Subcommands under `classroom materials`.
#[derive(Subcommand, Debug)]
pub enum MaterialsCommands {
    /// List course materials
    List {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Get a course material
    Get {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Material ID
        id: String,
    },

    /// Create a course material
    Create {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Title
        #[arg(long)]
        title: String,

        /// Description
        #[arg(long)]
        description: Option<String>,

        /// Topic ID
        #[arg(long)]
        topic_id: Option<String>,

        /// State (PUBLISHED, DRAFT)
        #[arg(long)]
        state: Option<String>,
    },

    /// Update a course material
    Update {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Material ID
        id: String,

        /// New title
        #[arg(long)]
        title: Option<String>,

        /// New description
        #[arg(long)]
        description: Option<String>,

        /// New state
        #[arg(long)]
        state: Option<String>,
    },

    /// Delete a course material
    Delete {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Material ID
        id: String,
    },
}

/// Subcommands under `classroom submissions`.
#[derive(Subcommand, Debug)]
pub enum SubmissionsCommands {
    /// List student submissions
    List {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Coursework ID
        #[arg(long)]
        coursework_id: String,

        /// Filter by user ID
        #[arg(long)]
        user_id: Option<String>,

        /// Filter by state
        #[arg(long)]
        state: Option<String>,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Get a student submission
    Get {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Coursework ID
        #[arg(long)]
        coursework_id: String,

        /// Submission ID
        id: String,
    },

    /// Grade a student submission
    Grade {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Coursework ID
        #[arg(long)]
        coursework_id: String,

        /// Submission ID
        id: String,

        /// Assigned grade
        #[arg(long)]
        grade: f64,
    },

    /// Return a student submission
    Return {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Coursework ID
        #[arg(long)]
        coursework_id: String,

        /// Submission ID
        id: String,
    },
}

/// Subcommands under `classroom announcements`.
#[derive(Subcommand, Debug)]
pub enum AnnouncementsCommands {
    /// List announcements
    List {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Get an announcement
    Get {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Announcement ID
        id: String,
    },

    /// Create an announcement
    Create {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Announcement text
        #[arg(long)]
        text: String,

        /// State (PUBLISHED, DRAFT)
        #[arg(long)]
        state: Option<String>,
    },

    /// Update an announcement
    Update {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Announcement ID
        id: String,

        /// New text
        #[arg(long)]
        text: Option<String>,

        /// New state
        #[arg(long)]
        state: Option<String>,
    },

    /// Delete an announcement
    Delete {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Announcement ID
        id: String,
    },
}

/// Subcommands under `classroom topics`.
#[derive(Subcommand, Debug)]
pub enum TopicsCommands {
    /// List topics
    List {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Get a topic
    Get {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Topic ID
        id: String,
    },

    /// Create a topic
    Create {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Topic name
        #[arg(long)]
        name: String,
    },

    /// Update a topic
    Update {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Topic ID
        id: String,

        /// New topic name
        #[arg(long)]
        name: String,
    },

    /// Delete a topic
    Delete {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// Topic ID
        id: String,
    },
}

/// Subcommands under `classroom invitations`.
#[derive(Subcommand, Debug)]
pub enum InvitationsCommands {
    /// List invitations
    List {
        /// Course ID
        #[arg(long)]
        course_id: Option<String>,

        /// User ID or email
        #[arg(long)]
        user_id: Option<String>,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Get an invitation
    Get {
        /// Invitation ID
        id: String,
    },

    /// Create an invitation
    Create {
        /// Course ID
        #[arg(long)]
        course_id: String,

        /// User ID or email
        #[arg(long)]
        user_id: String,

        /// Role (STUDENT, TEACHER, OWNER)
        #[arg(long)]
        role: String,
    },

    /// Accept an invitation
    Accept {
        /// Invitation ID
        id: String,
    },

    /// Delete an invitation
    Delete {
        /// Invitation ID
        id: String,
    },
}

/// Subcommands under `classroom guardians`.
#[derive(Subcommand, Debug)]
pub enum GuardiansCommands {
    /// List guardians for a student
    List {
        /// Student user ID or email
        #[arg(long)]
        student_id: String,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Get a guardian
    Get {
        /// Student user ID or email
        #[arg(long)]
        student_id: String,

        /// Guardian ID
        guardian_id: String,
    },

    /// Remove a guardian
    Delete {
        /// Student user ID or email
        #[arg(long)]
        student_id: String,

        /// Guardian ID
        guardian_id: String,
    },
}

/// Subcommands under `classroom guardian-invitations`.
#[derive(Subcommand, Debug)]
pub enum GuardianInvitationsCommands {
    /// List guardian invitations
    List {
        /// Student user ID or email
        #[arg(long)]
        student_id: String,

        /// Filter by state
        #[arg(long)]
        state: Option<String>,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Get a guardian invitation
    Get {
        /// Student user ID or email
        #[arg(long)]
        student_id: String,

        /// Invitation ID
        invitation_id: String,
    },

    /// Create a guardian invitation
    Create {
        /// Student user ID or email
        #[arg(long)]
        student_id: String,

        /// Guardian email
        #[arg(long)]
        guardian_email: String,
    },

    /// Cancel a guardian invitation
    Cancel {
        /// Student user ID or email
        #[arg(long)]
        student_id: String,

        /// Invitation ID
        invitation_id: String,
    },
}

/// Subcommands under `classroom profile`.
#[derive(Subcommand, Debug)]
pub enum ProfileCommands {
    /// Get a user profile
    Get {
        /// User ID or email
        user_id: String,
    },
}
