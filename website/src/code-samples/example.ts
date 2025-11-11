// TypeScript Example - API Client with Generics and Async/Await

interface User {
  id: number;
  username: string;
  email: string;
  createdAt: Date;
  isActive: boolean;
}

interface Post {
  id: number;
  title: string;
  content: string;
  authorId: number;
  tags: string[];
  publishedAt?: Date;
}

// Generic API response wrapper
interface ApiResponse<T> {
  data: T;
  status: number;
  message?: string;
}

// Generic paginated response
interface PaginatedResponse<T> {
  items: T[];
  total: number;
  page: number;
  pageSize: number;
  hasNext: boolean;
}

// Custom error class
class ApiError extends Error {
  constructor(
    public statusCode: number,
    message: string,
    public details?: unknown
  ) {
    super(message);
    this.name = 'ApiError';
  }
}

// Generic repository interface with CRUD operations
interface Repository<T, ID = number> {
  findById(id: ID): Promise<T | null>;
  findAll(options?: QueryOptions): Promise<T[]>;
  create(data: Omit<T, 'id'>): Promise<T>;
  update(id: ID, data: Partial<T>): Promise<T>;
  delete(id: ID): Promise<void>;
}

interface QueryOptions {
  page?: number;
  limit?: number;
  sortBy?: string;
  sortOrder?: 'asc' | 'desc';
  filters?: Record<string, unknown>;
}

// API Client with generics
class ApiClient<T> {
  private baseUrl: string;
  private headers: Record<string, string>;

  constructor(baseUrl: string, token?: string) {
    this.baseUrl = baseUrl;
    this.headers = {
      'Content-Type': 'application/json',
      ...(token && { Authorization: `Bearer ${token}` }),
    };
  }

  async get<R = T>(endpoint: string): Promise<ApiResponse<R>> {
    try {
      const response = await fetch(`${this.baseUrl}${endpoint}`, {
        method: 'GET',
        headers: this.headers,
      });

      if (!response.ok) {
        throw new ApiError(
          response.status,
          `Failed to fetch: ${response.statusText}`
        );
      }

      const data = await response.json();
      return {
        data,
        status: response.status,
      };
    } catch (error) {
      if (error instanceof ApiError) {
        throw error;
      }
      throw new ApiError(500, 'Network error', error);
    }
  }

  async post<R = T>(endpoint: string, body: unknown): Promise<ApiResponse<R>> {
    const response = await fetch(`${this.baseUrl}${endpoint}`, {
      method: 'POST',
      headers: this.headers,
      body: JSON.stringify(body),
    });

    const data = await response.json();
    return { data, status: response.status };
  }

  async put<R = T>(endpoint: string, body: unknown): Promise<ApiResponse<R>> {
    const response = await fetch(`${this.baseUrl}${endpoint}`, {
      method: 'PUT',
      headers: this.headers,
      body: JSON.stringify(body),
    });

    const data = await response.json();
    return { data, status: response.status };
  }

  async delete(endpoint: string): Promise<void> {
    await fetch(`${this.baseUrl}${endpoint}`, {
      method: 'DELETE',
      headers: this.headers,
    });
  }
}

// User service with type-safe methods
class UserService {
  private client: ApiClient<User>;

  constructor(baseUrl: string, token?: string) {
    this.client = new ApiClient<User>(baseUrl, token);
  }

  async getUser(id: number): Promise<User | null> {
    try {
      const response = await this.client.get<User>(`/users/${id}`);
      return response.data;
    } catch (error) {
      if (error instanceof ApiError && error.statusCode === 404) {
        return null;
      }
      throw error;
    }
  }

  async createUser(userData: Omit<User, 'id' | 'createdAt'>): Promise<User> {
    const response = await this.client.post<User>('/users', {
      ...userData,
      createdAt: new Date(),
    });
    return response.data;
  }

  async updateUser(id: number, updates: Partial<User>): Promise<User> {
    const response = await this.client.put<User>(`/users/${id}`, updates);
    return response.data;
  }

  async deleteUser(id: number): Promise<void> {
    await this.client.delete(`/users/${id}`);
  }

  async searchUsers(query: string): Promise<User[]> {
    const response = await this.client.get<User[]>(
      `/users/search?q=${encodeURIComponent(query)}`
    );
    return response.data;
  }
}

// Utility type helpers
type DeepPartial<T> = {
  [P in keyof T]?: T[P] extends object ? DeepPartial<T[P]> : T[P];
};

type ReadOnly<T> = {
  readonly [P in keyof T]: T[P];
};

// Advanced async patterns
async function processUsersBatch(
  userIds: number[],
  service: UserService
): Promise<User[]> {
  const userPromises = userIds.map((id) => service.getUser(id));

  // Process all requests in parallel
  const results = await Promise.allSettled(userPromises);

  // Filter successful results
  return results
    .filter((result): result is PromiseFulfilledResult<User> =>
      result.status === 'fulfilled' && result.value !== null
    )
    .map((result) => result.value);
}

// Retry logic with exponential backoff
async function withRetry<T>(
  fn: () => Promise<T>,
  maxRetries = 3,
  delay = 1000
): Promise<T> {
  let lastError: Error | undefined;

  for (let i = 0; i < maxRetries; i++) {
    try {
      return await fn();
    } catch (error) {
      lastError = error as Error;
      if (i < maxRetries - 1) {
        await new Promise((resolve) => setTimeout(resolve, delay * 2 ** i));
      }
    }
  }

  throw lastError || new Error('Max retries reached');
}

// Example usage
async function main() {
  const service = new UserService('https://api.example.com', 'my-token');

  try {
    // Create a new user
    const newUser = await service.createUser({
      username: 'johndoe',
      email: 'john@example.com',
      isActive: true,
    });
    console.log('Created user:', newUser);

    // Get user with retry
    const user = await withRetry(() => service.getUser(newUser.id));
    console.log('Fetched user:', user);

    // Process multiple users
    const users = await processUsersBatch([1, 2, 3, 4, 5], service);
    console.log(`Processed ${users.length} users`);

    // Update user
    if (user) {
      const updated = await service.updateUser(user.id, {
        isActive: false,
      });
      console.log('Updated user:', updated);
    }
  } catch (error) {
    if (error instanceof ApiError) {
      console.error(`API Error ${error.statusCode}:`, error.message);
    } else {
      console.error('Unexpected error:', error);
    }
  }
}

// Export for use in other modules
export {
  User,
  Post,
  ApiResponse,
  PaginatedResponse,
  ApiError,
  Repository,
  ApiClient,
  UserService,
  processUsersBatch,
  withRetry,
};
