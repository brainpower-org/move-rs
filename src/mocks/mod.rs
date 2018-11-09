use mocktopus::macros::*;
use rusoto_dynamodb::*;
use rusoto_core::RusotoFuture;

#[derive(Debug)]
pub struct DynamoDbMock;

#[mockable]
impl DynamoDb for DynamoDbMock {
    fn batch_get_item(
        &self,
        input: BatchGetItemInput,
    ) -> RusotoFuture<BatchGetItemOutput, BatchGetItemError> {
        unimplemented!();
    }
    fn batch_write_item(
        &self,
        input: BatchWriteItemInput,
    ) -> RusotoFuture<BatchWriteItemOutput, BatchWriteItemError> {
        unimplemented!();
    }
    fn create_backup(
        &self,
        input: CreateBackupInput,
    ) -> RusotoFuture<CreateBackupOutput, CreateBackupError> {
        unimplemented!();
    }
    fn create_global_table(
        &self,
        input: CreateGlobalTableInput,
    ) -> RusotoFuture<CreateGlobalTableOutput, CreateGlobalTableError> {
        unimplemented!();
    }
    fn create_table(
        &self,
        input: CreateTableInput,
    ) -> RusotoFuture<CreateTableOutput, CreateTableError> {
        unimplemented!();
    }
    fn delete_backup(
        &self,
        input: DeleteBackupInput,
    ) -> RusotoFuture<DeleteBackupOutput, DeleteBackupError> {
        unimplemented!();
    }
    fn delete_item(
        &self,
        input: DeleteItemInput,
    ) -> RusotoFuture<DeleteItemOutput, DeleteItemError> {
        unimplemented!();
    }
    fn delete_table(
        &self,
        input: DeleteTableInput,
    ) -> RusotoFuture<DeleteTableOutput, DeleteTableError> {
        unimplemented!();
    }
    fn describe_backup(
        &self,
        input: DescribeBackupInput,
    ) -> RusotoFuture<DescribeBackupOutput, DescribeBackupError> {
        unimplemented!();
    }

    fn describe_continuous_backups(
        &self,
        input: DescribeContinuousBackupsInput,
    ) -> RusotoFuture<DescribeContinuousBackupsOutput, DescribeContinuousBackupsError> {
        unimplemented!();
    }

    fn describe_global_table(
        &self,
        input: DescribeGlobalTableInput,
    ) -> RusotoFuture<DescribeGlobalTableOutput, DescribeGlobalTableError> {
        unimplemented!();
    }

    fn describe_global_table_settings(
        &self,
        input: DescribeGlobalTableSettingsInput,
    ) -> RusotoFuture<DescribeGlobalTableSettingsOutput, DescribeGlobalTableSettingsError> {
        unimplemented!();
    }

    fn describe_limits(&self) -> RusotoFuture<DescribeLimitsOutput, DescribeLimitsError> {
        unimplemented!();
    }

    fn describe_table(
        &self,
        input: DescribeTableInput,
    ) -> RusotoFuture<DescribeTableOutput, DescribeTableError> {
        unimplemented!();
    }

    fn describe_time_to_live(
        &self,
        input: DescribeTimeToLiveInput,
    ) -> RusotoFuture<DescribeTimeToLiveOutput, DescribeTimeToLiveError> {
        unimplemented!();
    }

    fn get_item(&self, input: GetItemInput) -> RusotoFuture<GetItemOutput, GetItemError> {
        unimplemented!();
    }

    fn list_backups(
        &self,
        input: ListBackupsInput,
    ) -> RusotoFuture<ListBackupsOutput, ListBackupsError> {
        unimplemented!();
    }
    fn list_global_tables(
        &self,
        input: ListGlobalTablesInput,
    ) -> RusotoFuture<ListGlobalTablesOutput, ListGlobalTablesError> {
        unimplemented!();
    }

    fn list_tables(
        &self,
        input: ListTablesInput,
    ) -> RusotoFuture<ListTablesOutput, ListTablesError> {
        unimplemented!();
    }

    fn list_tags_of_resource(
        &self,
        input: ListTagsOfResourceInput,
    ) -> RusotoFuture<ListTagsOfResourceOutput, ListTagsOfResourceError> {
        unimplemented!();
    }

    fn put_item(&self, input: PutItemInput) -> RusotoFuture<PutItemOutput, PutItemError> {
        unimplemented!();
    }

    fn query(&self, input: QueryInput) -> RusotoFuture<QueryOutput, QueryError> {
        unimplemented!();
    }

    fn restore_table_from_backup(
        &self,
        input: RestoreTableFromBackupInput,
    ) -> RusotoFuture<RestoreTableFromBackupOutput, RestoreTableFromBackupError> {
        unimplemented!();
    }
    fn restore_table_to_point_in_time(
        &self,
        input: RestoreTableToPointInTimeInput,
    ) -> RusotoFuture<RestoreTableToPointInTimeOutput, RestoreTableToPointInTimeError> {
        unimplemented!();
    }

    fn scan(&self, input: ScanInput) -> RusotoFuture<ScanOutput, ScanError> {
        unimplemented!();
    }
    fn tag_resource(&self, input: TagResourceInput) -> RusotoFuture<(), TagResourceError> {
        unimplemented!();
    }
    fn untag_resource(&self, input: UntagResourceInput) -> RusotoFuture<(), UntagResourceError> {
        unimplemented!();
    }

    fn update_continuous_backups(
        &self,
        input: UpdateContinuousBackupsInput,
    ) -> RusotoFuture<UpdateContinuousBackupsOutput, UpdateContinuousBackupsError> {
        unimplemented!();
    }

    fn update_global_table(
        &self,
        input: UpdateGlobalTableInput,
    ) -> RusotoFuture<UpdateGlobalTableOutput, UpdateGlobalTableError> {
        unimplemented!();
    }

    fn update_global_table_settings(
        &self,
        input: UpdateGlobalTableSettingsInput,
    ) -> RusotoFuture<UpdateGlobalTableSettingsOutput, UpdateGlobalTableSettingsError> {
        unimplemented!();
    }
    fn update_item(
        &self,
        input: UpdateItemInput,
    ) -> RusotoFuture<UpdateItemOutput, UpdateItemError> {
        unimplemented!();
    }

    fn update_table(
        &self,
        input: UpdateTableInput,
    ) -> RusotoFuture<UpdateTableOutput, UpdateTableError> {
        unimplemented!();
    }

    fn update_time_to_live(
        &self,
        input: UpdateTimeToLiveInput,
    ) -> RusotoFuture<UpdateTimeToLiveOutput, UpdateTimeToLiveError> {
        unimplemented!();
    }
}
