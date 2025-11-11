import Combine
import SwiftUI

// MARK: - Models

/// User model with Codable conformance
struct User: Identifiable, Codable, Equatable {
    let id: UUID
    var username: String
    var email: String
    var profileImageURL: URL?
    var createdAt: Date
    var isActive: Bool

    enum CodingKeys: String, CodingKey {
        case id
        case username
        case email
        case profileImageURL = "profile_image_url"
        case createdAt = "created_at"
        case isActive = "is_active"
    }
}

/// Post model with optional published date
struct Post: Identifiable, Codable {
    let id: UUID
    var title: String
    var content: String
    var authorId: UUID
    var tags: [String]
    var publishedAt: Date?
    var likes: Int

    var isPublished: Bool {
        publishedAt != nil
    }
}

// MARK: - Protocols

/// Protocol for API-fetchable resources
protocol Fetchable {
    associatedtype ID: Hashable
    var id: ID { get }
}

/// Repository protocol with generic type constraints
protocol Repository {
    associatedtype Item: Fetchable

    func fetch(id: Item.ID) async throws -> Item
    func fetchAll() async throws -> [Item]
    func create(_ item: Item) async throws -> Item
    func update(_ item: Item) async throws -> Item
    func delete(id: Item.ID) async throws
}

// MARK: - Errors

enum APIError: Error, LocalizedError {
    case networkError(Error)
    case invalidResponse
    case notFound
    case unauthorized
    case serverError(statusCode: Int)
    case decodingError(Error)

    var errorDescription: String? {
        switch self {
        case .networkError(let error):
            return "Network error: \(error.localizedDescription)"
        case .invalidResponse:
            return "Invalid response from server"
        case .notFound:
            return "Resource not found"
        case .unauthorized:
            return "Unauthorized access"
        case .serverError(let code):
            return "Server error with status code: \(code)"
        case .decodingError(let error):
            return "Failed to decode response: \(error.localizedDescription)"
        }
    }
}

// MARK: - API Client

class APIClient: ObservableObject {
    @Published var isLoading = false
    @Published var error: APIError?

    private let baseURL: URL
    private let session: URLSession

    init(baseURL: URL, session: URLSession = .shared) {
        self.baseURL = baseURL
        self.session = session
    }

    func request<T: Decodable>(
        endpoint: String,
        method: HTTPMethod = .get,
        body: Encodable? = nil
    ) async throws -> T {
        var request = URLRequest(url: baseURL.appendingPathComponent(endpoint))
        request.httpMethod = method.rawValue
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")

        if let body = body {
            request.httpBody = try JSONEncoder().encode(body)
        }

        do {
            let (data, response) = try await session.data(for: request)

            guard let httpResponse = response as? HTTPURLResponse else {
                throw APIError.invalidResponse
            }

            guard (200...299).contains(httpResponse.statusCode) else {
                throw APIError.serverError(statusCode: httpResponse.statusCode)
            }

            let decoder = JSONDecoder()
            decoder.dateDecodingStrategy = .iso8601
            return try decoder.decode(T.self, from: data)
        } catch let error as APIError {
            throw error
        } catch let error as DecodingError {
            throw APIError.decodingError(error)
        } catch {
            throw APIError.networkError(error)
        }
    }
}

enum HTTPMethod: String {
    case get = "GET"
    case post = "POST"
    case put = "PUT"
    case delete = "DELETE"
    case patch = "PATCH"
}

// MARK: - User Service

@MainActor
class UserService: ObservableObject {
    @Published var users: [User] = []
    @Published var currentUser: User?
    @Published var isLoading = false

    private let apiClient: APIClient

    init(apiClient: APIClient) {
        self.apiClient = apiClient
    }

    func fetchUser(id: UUID) async throws -> User {
        isLoading = true
        defer { isLoading = false }

        let user: User = try await apiClient.request(endpoint: "users/\(id.uuidString)")
        return user
    }

    func createUser(username: String, email: String) async throws -> User {
        let newUser = User(
            id: UUID(),
            username: username,
            email: email,
            profileImageURL: nil,
            createdAt: Date(),
            isActive: true
        )

        let created: User = try await apiClient.request(
            endpoint: "users",
            method: .post,
            body: newUser
        )
        users.append(created)
        return created
    }

    func updateUser(_ user: User) async throws {
        let updated: User = try await apiClient.request(
            endpoint: "users/\(user.id.uuidString)",
            method: .put,
            body: user
        )

        if let index = users.firstIndex(where: { $0.id == updated.id }) {
            users[index] = updated
        }
    }
}

// MARK: - SwiftUI Views

struct UserListView: View {
    @StateObject private var userService: UserService
    @State private var selectedUser: User?
    @State private var showError = false

    init(apiClient: APIClient) {
        _userService = StateObject(wrappedValue: UserService(apiClient: apiClient))
    }

    var body: some View {
        NavigationView {
            List(userService.users) { user in
                UserRowView(user: user)
                    .onTapGesture {
                        selectedUser = user
                    }
            }
            .navigationTitle("Users")
            .toolbar {
                ToolbarItem(placement: .primaryAction) {
                    Button(action: addUser) {
                        Image(systemName: "plus")
                    }
                }
            }
            .sheet(item: $selectedUser) { user in
                UserDetailView(user: user)
            }
        }
    }

    private func addUser() {
        Task {
            do {
                _ = try await userService.createUser(
                    username: "newuser",
                    email: "user@example.com"
                )
            } catch {
                showError = true
            }
        }
    }
}

struct UserRowView: View {
    let user: User

    var body: some View {
        HStack(spacing: 12) {
            // Profile image with optional handling
            if let imageURL = user.profileImageURL {
                AsyncImage(url: imageURL) { image in
                    image
                        .resizable()
                        .aspectRatio(contentMode: .fill)
                } placeholder: {
                    ProgressView()
                }
                .frame(width: 50, height: 50)
                .clipShape(Circle())
            } else {
                Circle()
                    .fill(Color.gray.opacity(0.3))
                    .frame(width: 50, height: 50)
                    .overlay(
                        Text(user.username.prefix(1))
                            .font(.title2)
                            .foregroundColor(.white)
                    )
            }

            VStack(alignment: .leading, spacing: 4) {
                Text(user.username)
                    .font(.headline)

                Text(user.email)
                    .font(.subheadline)
                    .foregroundColor(.secondary)
            }

            Spacer()

            if user.isActive {
                Circle()
                    .fill(Color.green)
                    .frame(width: 8, height: 8)
            }
        }
        .padding(.vertical, 4)
    }
}

struct UserDetailView: View {
    let user: User
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        NavigationView {
            Form {
                Section("Profile") {
                    LabeledContent("Username", value: user.username)
                    LabeledContent("Email", value: user.email)
                    LabeledContent("Status", value: user.isActive ? "Active" : "Inactive")
                }

                Section("Details") {
                    LabeledContent("User ID", value: user.id.uuidString)
                    LabeledContent("Created", value: user.createdAt.formatted())
                }
            }
            .navigationTitle("User Details")
            .toolbar {
                ToolbarItem(placement: .confirmationAction) {
                    Button("Done") {
                        dismiss()
                    }
                }
            }
        }
    }
}

// MARK: - Preview Provider

#Preview {
    let mockClient = APIClient(baseURL: URL(string: "https://api.example.com")!)
    return UserListView(apiClient: mockClient)
}
