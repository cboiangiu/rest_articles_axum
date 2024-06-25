package com.jetbrains.kmpapp.data

import io.ktor.client.HttpClient
import io.ktor.client.call.body
import io.ktor.client.request.get
import kotlin.coroutines.cancellation.CancellationException
import kotlinx.serialization.Serializable

interface ArticleApi {
    suspend fun getData(): List<ArticleObject>
}

class KtorArticleApi(private val client: HttpClient) : ArticleApi {
    companion object {
        private const val API_URL =
//            "http://localhost:3000/api/v1/article"
            "http://10.0.2.2:3000/api/v1/article" // for android emulator
    }

    override suspend fun getData(): List<ArticleObject> {
        return try {
            client.get("$API_URL?pageNumber=1&pageSize=1000").body<ArticleApiResponse>().items
        } catch (e: Exception) {
            if (e is CancellationException) throw e
            e.printStackTrace()

            emptyList()
        }
    }
}

@Serializable
private data class ArticleApiResponse(
    val items: List<ArticleObject>
)
