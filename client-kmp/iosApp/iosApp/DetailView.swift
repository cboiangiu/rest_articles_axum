import Foundation
import SwiftUI
import shared

struct DetailView: View {
    let viewModel = DetailViewModel(
        articleRepository: KoinDependencies().articleRepository
    )
    
    let id: String
    
    @State
    var object: ArticleObject? = nil
    
    var body: some View {
        VStack {
            if let obj = object {
                ObjectDetails(obj: obj)
            }
        }.task {
            for await obj in viewModel.getObject(objectId: id) {
                object = obj!
            }
        }
    }
}

struct ObjectDetails: View {
    var obj: ArticleObject
    
    var body: some View {
        ScrollView {
            
            VStack {
                AsyncImage(url: URL(string: "https://images.pexels.com/photos/3225517/pexels-photo-3225517.jpeg")) { phase in
                    switch phase {
                    case .empty:
                        ProgressView()
                    case .success(let image):
                        image
                            .resizable()
                            .scaledToFill()
                            .clipped()
                    default:
                        EmptyView()
                    }
                }
                
                VStack(alignment: .leading, spacing: 6) {
                    Text(obj.title)
                        .font(.title)
                    
                    LabeledInfo(label: "Content", data: obj.content)
                    LabeledInfo(label: "PublishedDate", data: obj.publishedDate.epochSeconds.description)
                }
                .padding(16)
            }
        }
    }
}

struct LabeledInfo: View {
    var label: String
    var data: String
    
    var body: some View {
        Spacer()
        Text("**\(label):** \(data)")
    }
}
