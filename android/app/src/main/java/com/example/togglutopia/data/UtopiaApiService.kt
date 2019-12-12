package com.example.togglutopia.data

import com.example.togglutopia.data.model.request.SyncRequest
import com.example.togglutopia.data.model.response.SnapshotResponse
import com.example.togglutopia.data.model.response.SyncResponse
import retrofit2.Call
import retrofit2.http.Body
import retrofit2.http.GET
import retrofit2.http.Header
import retrofit2.http.POST

interface UtopiaApiService {
    @GET("/current-snapshot")
    fun login(@Header("Authorization") credentials: String): Call<SnapshotResponse>

    @POST("/sync")
    fun sync(@Header("Authorization") credentials: String, @Body requestBody: SyncRequest): Call<SyncResponse>
}