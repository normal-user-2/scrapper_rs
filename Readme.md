## Learning Rust with Scrapping Project

### Goal

- [x] Setup SQLx
- [ ] Scrap any wordpress blog
- [ ] Save into database (page model/table)
- [ ] Extract article data (post model/table)
  - [ ] Normalized body
  - [ ] Extracted title
  - [ ] Extracted thumbnail_image
  - [ ] Extracted category
  - [ ] Extracted author
- [ ] Setup Axum
  - [ ] Create API to get the data
- [ ] Setup Actix
  - [ ] Create API to get the data
- [ ] Compare the performance between Axum and Actix

### Setup

- Create `.env` file
  ```env
  DATABASE_URL=postgresql://postgres:password@localhost:5432/learning_rust
  ENVIRONMENT=dev
  ```
