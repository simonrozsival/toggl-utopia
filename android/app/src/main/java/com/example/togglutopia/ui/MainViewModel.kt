package com.example.togglutopia.ui

import android.app.Application
import androidx.lifecycle.*
import com.example.togglutopia.data.Repository
import kotlinx.coroutines.ExperimentalCoroutinesApi

class MainViewModel(app: Application) : AndroidViewModel(app), MainInteractions {

    private val repository = Repository(getApplication())

    override fun login(username: String, password: String) {
        repository.login(username, password)
    }

    override fun sync() {
        repository.sync()
    }
}

interface MainInteractions : LoginInteractions, LogInteractions

interface LoginInteractions {
    fun login(username: String, password: String)
}

interface LogInteractions {
    fun sync()
}