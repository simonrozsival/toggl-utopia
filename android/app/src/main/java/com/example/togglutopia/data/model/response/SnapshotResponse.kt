package com.example.togglutopia.data.model.response

import com.example.togglutopia.data.model.Meta
import com.example.togglutopia.data.model.Project
import com.example.togglutopia.data.model.TimeEntry
import com.example.togglutopia.data.model.User

data class SnapshotResponse(
        val meta: Meta,
        val payload: SnapshotPayload
)

data class SnapshotPayload(
        val projects: List<Project>,
        val time_entries: List<TimeEntry>,
        val user: User
)