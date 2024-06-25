package com.jetbrains.kmpapp.di

import co.touchlab.skie.configuration.annotations.DefaultArgumentInterop
import com.jetbrains.kmpapp.data.InMemoryArticleStorage
import com.jetbrains.kmpapp.data.KtorArticleApi
import com.jetbrains.kmpapp.data.ArticleApi
import com.jetbrains.kmpapp.data.ArticleRepository
import com.jetbrains.kmpapp.data.ArticleStorage
import io.ktor.client.HttpClient
import io.ktor.client.plugins.contentnegotiation.ContentNegotiation
import io.ktor.http.ContentType
import io.ktor.serialization.kotlinx.json.json
import kotlinx.serialization.json.Json
import org.koin.core.context.startKoin
import org.koin.core.module.Module
import org.koin.dsl.module

val dataModule = module {
    single {
        val json = Json { ignoreUnknownKeys = true }
        HttpClient {
            install(ContentNegotiation) {
                // TODO Fix API so it serves application/json
                json(json, contentType = ContentType.Any)
            }
        }
    }

    single<ArticleApi> { KtorArticleApi(get()) }
    single<ArticleStorage> { InMemoryArticleStorage() }
    single {
        ArticleRepository(get(), get()).apply {
            initialize()
        }
    }
}

@DefaultArgumentInterop.Enabled
fun initKoin(modules: List<Module> = emptyList()) {
    startKoin {
        modules(
            dataModule,
            *modules.toTypedArray(),
        )
    }
}
