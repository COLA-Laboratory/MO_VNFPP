addpath(".");

close all
clear
clc
format long g

%% Parameters
num_objectives = 3;

runs = 30;

% root_path = "/media/joebillingsley/Data/projects/NFV_PlacementModel_Journal";
root_path = "D:\Research\NFV_PlacementModel_Journal";

src_folder = fullfile(root_path, "processed", "aggregate", "LimitedLicenses");
out_folder = fullfile(root_path, "processed", "limited_licenses");

if exist(out_folder, 'dir')
    rmdir(out_folder, 's')
end

perc_licensed = ["10", "20", "30", "40", "50", "60", "70", "80", "90", "100",];
num_licensed = ["80", "20", "5"];

operators = ["ca", "std"];
algorithms = ["NSGAII"];

data = cell(length(operators), length(algorithms));

for i = 1:length(perc_licensed)
    for j = 1:length(num_licensed)
        for l = 1:length(operators)
            for k = 1:length(algorithms)
                
                perc = perc_licensed(i);
                num = num_licensed(j);
                alg = algorithms(k);
                op = operators(l);
                
                file = fullfile(src_folder, perc, num, op, alg, 'aggregate.csv');
                hvs = csvread(file);
                mean_final = hvs(end, 2);
                
                data{k, l}(i, j) = mean_final;
            end
        end
    end
end

for i = 1:length(algorithms)
    for j = 1:length(operators)
        
        algorithm = algorithms(i);
        operator = operators(j);
        
        dest_folder = fullfile(out_folder, operator, algorithm);
        if ~exist(dest_folder, 'dir')
            mkdir(dest_folder);
        end
        
        out_file = fullfile(dest_folder, 'heatmap.csv');
        
        writematrix(data{i, j}, out_file);
    end
end