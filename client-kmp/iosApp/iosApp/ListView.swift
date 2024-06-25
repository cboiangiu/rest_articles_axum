import SwiftUI
import KMMViewModelSwiftUI
import shared

struct ListView: View {
    let viewModel = ListViewModel(
        articleRepository: KoinDependencies().articleRepository
    )
    
    @State
    var objects: [ArticleObject] = []
    
    let columns = [
        GridItem(.adaptive(minimum: 120), alignment: .top)
    ]
    
    var body: some View {
        ZStack {
            if !objects.isEmpty {
                NavigationStack {
                    ScrollView {
                        LazyVGrid(columns: columns, alignment: .leading, spacing: 20) {
                            ForEach(objects, id: \.id) { item in
                                NavigationLink(destination: DetailView(id: item.id)) {
                                    ObjectFrame(obj: item, onClick: {})
                                }
                                .buttonStyle(PlainButtonStyle())
                            }
                        }
                        .padding(.horizontal)
                    }
                }
            } else {
                Text("No data available")
            }
        }.task {
            for await objs in viewModel.objects {
                objects = objs
            }
        }
    }
}

struct ObjectFrame: View {
    let obj: ArticleObject
    let onClick: () -> Void
    
    var body: some View {
        VStack(alignment: .leading, spacing: 4) {
            GeometryReader { geometry in
                AsyncImage(url: URL(string: "https://images.pexels.com/photos/3225517/pexels-photo-3225517.jpeg")) { phase in
                    switch phase {
                    case .empty:
                        ProgressView()
                            .frame(width: geometry.size.width, height: geometry.size.width)
                    case .success(let image):
                        image
                            .resizable()
                            .scaledToFill()
                            .frame(width: geometry.size.width, height: geometry.size.width)
                            .clipped()
                            .aspectRatio(1, contentMode: .fill)
                    default:
                        EmptyView()
                            .frame(width: geometry.size.width, height: geometry.size.width)
                    }
                }
            }
            .aspectRatio(1, contentMode: .fit)
            
            Text(obj.title)
                .font(.headline)
            
            Text(obj.publishedDate.epochSeconds.description)
                .font(.caption)
        }
    }
}
