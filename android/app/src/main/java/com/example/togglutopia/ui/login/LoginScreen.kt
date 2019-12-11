package com.example.togglutopia.ui.login

import android.content.ContentValues.TAG
import android.util.Log
import androidx.compose.*
import androidx.ui.core.Modifier
import androidx.ui.core.Text
import androidx.ui.core.TextField
import androidx.ui.core.dp
import androidx.ui.layout.Column
import androidx.ui.layout.Padding
import androidx.ui.material.Button
import androidx.ui.material.MaterialTheme
import androidx.ui.material.surface.Card
import androidx.ui.tooling.preview.Preview
import com.example.togglutopia.ui.LoginInteractions

@Model
class LoginState(var username: String, var password: String)

@Composable
fun LoginScreen(interactions: LoginInteractions) {
    val loginState = +memo { LoginState("valenta@test.com", "testtest") }
    Padding(padding = 8.dp) {
        Card(elevation = 4.dp) {
            Padding(padding = 8.dp) {
                Column {
                    Text(text = "Login", style = ((+MaterialTheme.typography()).h6))
                    TextField(
                            value = loginState.username,
                            onValueChange = { loginState.username = it },
                            textStyle = ((+MaterialTheme.typography()).body2)
                    )
                    TextField(
                            value = loginState.password,
                            onValueChange = { loginState.password = it },
                            textStyle = ((+MaterialTheme.typography()).body2)
                    )
                    Button(text = "Login", onClick = { interactions.login(loginState.username, loginState.password) })
                }
            }
        }
    }
}

@Preview
@Composable
fun preview() {
    LoginScreen(object : LoginInteractions {
        override fun login(username: String, password: String) {}
    })
}