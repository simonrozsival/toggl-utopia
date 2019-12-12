package com.example.togglutopia.data

import com.example.togglutopia.data.model.SnapshotResponse
import retrofit2.Call
import retrofit2.http.GET
import retrofit2.http.Header
import retrofit2.http.POST

interface UtopiaApiService {
    @GET("/current-snapshot")
    fun login(@Header("Authorization") credentials: String): Call<SnapshotResponse>

    @GET("/sync")
    fun sync(@Header("Authorization") credentials: String): Call<SnapshotResponse>
}