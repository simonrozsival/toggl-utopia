package com.example.togglutopia.data.model.response

import com.example.togglutopia.data.model.*

data class SyncResponse(
        val meta: Meta,
        val payload: SyncPayload
)

data class SyncPayload(
        val user: EntityUpdate<User>,
        val projects: List<EntityUpdate<Project>>,
        val time_entries: List<EntityUpdate<TimeEntry>>
)