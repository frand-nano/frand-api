use crate::{
    db,
    error::{ApiError, BAD_REQUEST, INTERNAL_SERVER_ERROR, NOT_FOUND, VALIDATION_ERROR},
    models::memo::{CreateMemoRequest, Memo, UpdateMemoRequest},
    response::ApiResponse,
};
use log::{error, info};
use mongodb::{bson::oid::ObjectId, Database};
use rocket::{delete, get, post, put, serde::json::Json, State};
use validator::Validate;

#[post("/", data = "<req>")]
pub async fn create_memo_handler(
    db: &State<Database>,
    req: Json<CreateMemoRequest>,
) -> Result<Json<ApiResponse<Memo>>, ApiError> {
    // 입력값 유효성 검사
    if let Err(errors) = req.validate() {
        let error_details = serde_json::to_value(&errors).unwrap_or_default();
        return Err(ApiError::new(
            VALIDATION_ERROR,
            "입력값 유효성 검사 실패",
            rocket::http::Status::BadRequest,
        )
        .with_details(error_details));
    }

    // 메모 생성
    match db::create_memo_db(db, req.into_inner()).await {
        Ok(memo) => {
            info!("메모 생성됨: {:?}", memo.id);
            Ok(Json(ApiResponse::new(memo)))
        }
        Err(e) => {
            error!("메모 생성 실패: {}", e);
            Err(ApiError::new(
                INTERNAL_SERVER_ERROR,
                "메모 생성 중 오류 발생",
                rocket::http::Status::InternalServerError,
            ))
        }
    }
}

#[get("/")]
pub async fn list_memos_handler(db: &State<Database>) -> Result<Json<ApiResponse<Vec<Memo>>>, ApiError> {
    match db::list_memos_db(db).await {
        Ok(memos) => {
            info!("메모 목록 조회: {} 건", memos.len());
            Ok(Json(ApiResponse::new(memos)))
        }
        Err(e) => {
            error!("메모 목록 조회 실패: {}", e);
            Err(ApiError::new(
                INTERNAL_SERVER_ERROR,
                "메모 목록 조회 중 오류 발생",
                rocket::http::Status::InternalServerError,
            ))
        }
    }
}

#[get("/<id>")]
pub async fn get_memo_handler(
    db: &State<Database>,
    id: &str,
) -> Result<Json<ApiResponse<Memo>>, ApiError> {
    // ID 문자열 -> ObjectId 변환
    let obj_id = match ObjectId::parse_str(id) {
        Ok(oid) => oid,
        Err(_) => {
            return Err(ApiError::new(
                BAD_REQUEST,
                "잘못된 ID 형식",
                rocket::http::Status::BadRequest,
            ))
        }
    };

    // 메모 조회
    match db::get_memo_db(db, obj_id).await {
        Ok(Some(memo)) => {
            info!("메모 조회: {}", id);
            Ok(Json(ApiResponse::new(memo)))
        }
        Ok(None) => {
            info!("존재하지 않는 메모 ID: {}", id);
            Err(ApiError::new(
                NOT_FOUND,
                "메모를 찾을 수 없습니다",
                rocket::http::Status::NotFound,
            ))
        }
        Err(e) => {
            error!("메모 조회 실패: {}", e);
            Err(ApiError::new(
                INTERNAL_SERVER_ERROR,
                "메모 조회 중 오류 발생",
                rocket::http::Status::InternalServerError,
            ))
        }
    }
}

#[put("/<id>", data = "<req>")]
pub async fn update_memo_handler(
    db: &State<Database>,
    id: &str,
    req: Json<UpdateMemoRequest>,
) -> Result<Json<ApiResponse<Memo>>, ApiError> {
    // 입력값 유효성 검사
    if let Err(errors) = req.validate() {
        let error_details = serde_json::to_value(&errors).unwrap_or_default();
        return Err(ApiError::new(
            VALIDATION_ERROR,
            "입력값 유효성 검사 실패",
            rocket::http::Status::BadRequest,
        )
        .with_details(error_details));
    }

    // ID 문자열 -> ObjectId 변환
    let obj_id = match ObjectId::parse_str(id) {
        Ok(oid) => oid,
        Err(_) => {
            return Err(ApiError::new(
                BAD_REQUEST,
                "잘못된 ID 형식",
                rocket::http::Status::BadRequest,
            ))
        }
    };

    // 메모 수정
    match db::update_memo_db(db, obj_id, req.into_inner()).await {
        Ok(Some(memo)) => {
            info!("메모 수정됨: {}", id);
            Ok(Json(ApiResponse::new(memo)))
        }
        Ok(None) => {
            info!("수정 시도 - 존재하지 않는 메모 ID: {}", id);
            Err(ApiError::new(
                NOT_FOUND,
                "메모를 찾을 수 없습니다",
                rocket::http::Status::NotFound,
            ))
        }
        Err(e) => {
            error!("메모 수정 실패: {}", e);
            Err(ApiError::new(
                INTERNAL_SERVER_ERROR,
                "메모 수정 중 오류 발생",
                rocket::http::Status::InternalServerError,
            ))
        }
    }
}

#[delete("/<id>")]
pub async fn delete_memo_handler(
    db: &State<Database>,
    id: &str,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    // ID 문자열 -> ObjectId 변환
    let obj_id = match ObjectId::parse_str(id) {
        Ok(oid) => oid,
        Err(_) => {
            return Err(ApiError::new(
                BAD_REQUEST,
                "잘못된 ID 형식",
                rocket::http::Status::BadRequest,
            ))
        }
    };

    // 메모 삭제
    match db::delete_memo_db(db, obj_id).await {
        Ok(deleted) => {
            if deleted {
                info!("메모 삭제됨: {}", id);
                Ok(Json(ApiResponse::new(())))
            } else {
                info!("삭제 시도했으나 메모가 없음: {}", id);
                Err(ApiError::new(
                    NOT_FOUND,
                    "메모를 찾을 수 없습니다",
                    rocket::http::Status::NotFound,
                ))
            }
        }
        Err(e) => {
            error!("메모 삭제 실패: {}", e);
            Err(ApiError::new(
                INTERNAL_SERVER_ERROR,
                "메모 삭제 중 오류 발생",
                rocket::http::Status::InternalServerError,
            ))
        }
    }
}
