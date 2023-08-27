namespace Microsoft.Extensions.DependencyInjection;

/// <summary>
/// Методы расширения для коллекции сервисов добавляемых в DI контейнер.
/// </summary>
internal static class ServiceCollectionExtensions
{
    /// <summary>
    /// Добавление конфигураций сервиса.
    /// </summary>
    /// <param name="services">Коллекция сервисов типа <see cref="IServiceCollection"/>.</param>
    /// <param name="configuration">Объект конфигураций типа <see cref="IConfiguration"/>.</param>
    /// <returns>Оригинальная коллекция сервисов <see cref="IServiceCollection"/>.</returns>
    public static IServiceCollection AddConfigurations(this IServiceCollection services, IConfiguration configuration)
    {
        return services;
    }
    
    /// <summary>
    /// Регистрация сервисов бизнес логики.
    /// </summary>
    /// <param name="services">Коллекция сервисов типа <see cref="IServiceCollection"/>.</param>
    /// <returns>Оригинальная коллекция сервисов <see cref="IServiceCollection"/>.</returns>
    public static IServiceCollection AddApplicationServices(this IServiceCollection services)
    {
        return services;
    }
}