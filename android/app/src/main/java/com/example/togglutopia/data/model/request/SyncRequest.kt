package com.example.togglutopia.data.model.request

import com.example.togglutopia.data.model.Delta

data class SyncRequest(
        val last_sync: String,
        val delta: Delta
)