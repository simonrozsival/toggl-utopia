package com.example.togglutopia.data

import android.content.Context
import android.util.Log
import com.example.togglutopia.data.model.Delta
import com.example.togglutopia.data.model.UpdateType
import com.example.togglutopia.data.model.request.SyncRequest
import com.example.togglutopia.data.model.response.SnapshotResponse
import com.example.togglutopia.data.model.response.SyncResponse
import com.example.togglutopia.ui.TogglState
import com.example.togglutopia.ui.localOnly
import okhttp3.Credentials
import okhttp3.OkHttpClient
import okhttp3.logging.HttpLoggingInterceptor
import retrofit2.Call
import retrofit2.Callback
import retrofit2.Response
import retrofit2.Retrofit
import retrofit2.converter.moshi.MoshiConverterFactory

class Repository(context: Context) {

    private val localDb by lazy { LocalDb(context) }

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

    fun restoreState() {
        localDb.restoreState()
    }

    fun persistState() {
        localDb.persistState()
    }

    fun login(username: String, password: String) {
        val credentials = Credentials.basic(username, password)
        api.login(credentials).enqueue(SnapshotResponseHandler {
            localDb.lastSync = it
            localDb.persistState()
        })
    }

    fun sync() {
        TogglState.user?.api_token?.let { apiToken ->
            val credentials = "Bearer $apiToken"
            api.sync(credentials, getSyncRequest()).enqueue(SyncResponseHandler { localDb.lastSync = it })
        }
    }

    private fun getSyncRequest(): SyncRequest {
        return SyncRequest(
                last_sync = localDb.lastSync ?: "",
                delta = Delta(
                    time_entries = TogglState.timeEntryList.filter { it.edited }
                )
        )
    }
}

class SyncResponseHandler(val onSuccess: (String) -> Unit) : Callback<SyncResponse> {
    override fun onFailure(call: Call<SyncResponse>, t: Throwable) {
        Log.d("SyncResponseHandler", "onFailure() called with: call = $call, t = $t")
    }

    override fun onResponse(call: Call<SyncResponse>, response: Response<SyncResponse>) {
        response.body()?.payload?.apply {
            TogglState.user = user.entity

//            TogglState.clearLocalTimeEntries()
            time_entries.forEach { entityUpdate ->
                when (entityUpdate.type) {
                    UpdateType.Changed -> {
                        TogglState.editTimeEntry(entityUpdate)
                    }
                    UpdateType.Created -> {
                        TogglState.editTimeEntry(entityUpdate)
                    }
                    UpdateType.Deleted -> {
                        TogglState.deleteTimeEntry(entityUpdate)
                    }
                }
            }
            response.body()?.meta?.utc_server_time?.let { onSuccess(it) }
        }
    }
}

class SnapshotResponseHandler(val onSuccess: (String) -> Unit) : Callback<SnapshotResponse> {
    override fun onFailure(call: Call<SnapshotResponse>, t: Throwable) {
        Log.d("SnapshotResponseHandler", "onFailure() called with: call = $call, t = $t")
    }

    override fun onResponse(call: Call<SnapshotResponse>, response: Response<SnapshotResponse>) {
        if (response.isSuccessful) {
            // this should be persisted in the database not the global UI state
            response.body()?.payload?.apply {
                user.let { TogglState.user = it }
                time_entries.let {
                    TogglState.timeEntryList.clear()
                    TogglState.timeEntryList.addAll(it)
                }
                projects.let {
                    TogglState.projectList.clear()
                    TogglState.projectList.addAll(it)
                }
            }

            response.body()?.meta?.utc_server_time?.let { onSuccess(it) }
        }
    }
}