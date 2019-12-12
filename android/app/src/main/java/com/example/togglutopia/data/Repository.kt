package com.example.togglutopia.data

import android.util.Log
import com.example.togglutopia.data.model.SnapshotResponse
import com.example.togglutopia.ui.TogglState
import kotlinx.coroutines.ExperimentalCoroutinesApi
import okhttp3.Credentials
import okhttp3.OkHttpClient
import okhttp3.logging.HttpLoggingInterceptor
import retrofit2.Call
import retrofit2.Callback
import retrofit2.Response
import retrofit2.Retrofit
import retrofit2.converter.moshi.MoshiConverterFactory

class Repository() {

    private val client by lazy {
        val interceptor = HttpLoggingInterceptor()
        interceptor.level = HttpLoggingInterceptor.Level.BODY
        return@lazy OkHttpClient.Builder().addInterceptor(interceptor).build()
    }

    private val retrofit by lazy {
        return@lazy Retrofit.Builder()
                .client(client)
                .baseUrl("http://10.0.2.2:8080")
                .addConverterFactory(MoshiConverterFactory.create())
                .build()
    }

    private val api by lazy { retrofit.create(UtopiaApiService::class.java)}

    fun login(username: String, password: String) {
        val credentials = Credentials.basic(username, password)
        api.login(credentials).enqueue(object : Callback<SnapshotResponse> {
            override fun onFailure(call: Call<SnapshotResponse>, t: Throwable) {
                Log.d("Repository", "onFailure() called with: call = $call, t = $t")
            }

            override fun onResponse(call: Call<SnapshotResponse>, response: Response<SnapshotResponse>) {
                handleSnapshot(call, response)
            }
        })
    }

    fun sync() {
        TogglState.user?.api_token?.let { apiToken ->
            api.sync("Bearer $apiToken").enqueue(object : Callback<SnapshotResponse> {
                override fun onFailure(call: Call<SnapshotResponse>, t: Throwable) {
                    Log.d("Repository", "onFailure() called with: call = $call, t = $t")
                }

                override fun onResponse(call: Call<SnapshotResponse>, response: Response<SnapshotResponse>) {
                    handleSnapshot(call, response)
                }
            })
        }
    }

    private fun handleSnapshot(call: Call<SnapshotResponse>, response: Response<SnapshotResponse>) {
        Log.d("Repository", "onResponse() called with: call = $call, response = $response")
        if (response.isSuccessful) {
            // this should be persisted in the database not the global UI state
            response.body()?.payload?.user?.let { TogglState.user = it }
            response.body()?.payload?.time_entries?.let {
                TogglState.timeEntryList.clear()
                TogglState.timeEntryList.addAll(it)
            }
            response.body()?.payload?.projects?.let {
                TogglState.projectList.clear()
                TogglState.projectList.addAll(it)
            }
        }
    }

}