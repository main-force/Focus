# Focus
rbac-based organizational business integrated management program

# RBAC 기반 조직 업무 통합관리 프로그램

# 프로젝트 중단 - 2021/06/24

## 이유

### 사용자가 게시판을 커스터마이징 한다는 개념 자체가 상당히 불편함.

- 내가 사용자에게 제공하는 **가치에 쉽게 접근할 수 있어야 함**. 하지만, 내가 제공할 서비스는 제공하는 가치의 접근이 어려움.
- 복잡한 과정들은 **프로그램이 처리**하고, 그 결과를 사용자에게 보여주는 흐름으로 가는게 좋은 것으로 판단됨.
- 현재 우리가 겪는 복잡한 과정들을 찾아볼 필요가 있음. 이 때, 이 복잡한 과정이 귀찮은 일이 아닌, 새로운 가치를 만들어내는 것이여야 함. (Ex. 누구에게 권한을 할당해야하나?[X], 당신이 모르는 음악이지만 좋아할 것을 추천합니다![O])

---

# 진행상황

### 예정

- 유저에게 역할 추가
- 글 쓰기

### 진행 중

### 완료

- 권한 리스트 출력
- 권한 리스트에 권한 추가
- 권한 리스트에 권한 삭제
- 역할 리스트에 역할 추가
- 역할 리스트에 역할 삭제
- 역할에 권한 추가
- 역할 리스트 출력
- 유저 리스트 출력
- 유저 리스트에 유저 추가
- 유저 리스트에 유저 삭제

---

# 명세

## 기능

### 온라인 게시판

- 역할에 따라 게시판의 컨텐츠를 볼 수 있음.
- 본인이 볼 수 있는 게시판은 해당 역할그룹에 알림을 전송할 수 있음.

### 문서 관리

- 역할에 따라 문서를 읽기, 쓰기를 할 수 있음.

### 개인 페이지 관리

- 본인의 권한에 맞는 정보들을 모두 확인 할 수 있음.

## 제약조건

## 목표

---

---

# 시스템

## 권한 관리 프로그램

- 권한 리스트 출력
- 권한 리스트에 권한 추가
- 권한 리스트에 권한 삭제

## 역할 관리 프로그램

- 역할 리스트 출력
- 역할 리스트에 역할 추가
- 역할 리스트에 역할 삭제
- 역할에 권한 추가

## 유저 관리 프로그램

- 유저 리스트 출력
- 유저 리스트에 유저 추가
- 유저 리스트에 유저 삭제
- 유저에게 역할 추가

---

### 유저

- 역할

### 역할[마인드맵 그려서 구성요소를 알아내야겠다]

- 권한

### 컨텐츠

- 요구권한

### 역할List-tree

- 역할들

### 권한List

- 권한들

## 입력 및 처리내용

## 파이프라인

### 컨텐츠 작성

글 작성 → 권한 부여 → 업로드

# 프로그래밍

- 사용 언어: Rust
